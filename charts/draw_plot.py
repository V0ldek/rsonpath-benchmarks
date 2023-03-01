import matplotlib.pyplot as plot
import numpy as np
import math

plot.rcParams.update({
    "font.size": 18,
    "axes.facecolor": "whitesmoke",
    "font.family": "serif"
})

def print_plot(rsonpath, jsurfer, jsonski, exp_label, fileout="plot.png"):
    width = 0.6
    ratio = 1.8
    pos = np.array(range(len(exp_label)))
    fig, (ax0, ax1) = plot.subplots(1, 2, gridspec_kw={'width_ratios':[1, ratio]})
    ax0.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    bar = ax0.bar(exp_label, jsurfer, width=width, label="jsurfer", color="tab:gray", zorder=3)
    ax0.legend()
    ax0.set_ylabel("GB/s")
    #ax0.bar_label(bar, [f"{e:0.2f}" for e in jsurfer])

    width = width/ratio

    bar = ax1.bar(pos+width/2+0.03, rsonpath, label="simdpath", width=width, color="tab:blue", zorder=4)
    ax1.set_xticks(pos)
    ax1.set_xticklabels(exp_label)
    ax1.bar_label(bar, [f"{e:0.0f}" for e in rsonpath/jsurfer])
    pos2, jsonski2 = zip(*filter(lambda e:e[1] > 0, zip(pos, jsonski)))
    jsonski2 = np.array(jsonski2)
    pos2 = np.array(pos2)

    bar = ax1.bar(pos2-width/2-0.03, jsonski2, label="jsonski", width=width, color="tab:red", zorder=4)
    ax1.bar_label(bar, [f"{e:0.0f}" for e in filter(bool, jsonski/jsurfer)], zorder=4)
    ax1.set_ylabel("GB/s")
    ax1.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    ax1.legend()
    fig.tight_layout()
    fig.set_size_inches(20, 5)
    plot.subplots_adjust(wspace=0.2, left=0.06)
    plot.savefig("plot.png")
    
def plot_from_dataframe(df,
	 keys=None,
	 width=0.8,
	 colors=dict(simdpath="tab:blue",
    	 jsonski="tab:red",
	     rewritten_s="tab:green",
         jsurfer="tab:gray",
         rewritten_j="tab:brown"
     ),
	 labels = dict(rewritten_s="simdpath (rewritten)", rewritten_j="jsurfer (rewritten)")):

    keys = list(df) if not keys else keys
    plot.rcParams.update({
    "font.size": 28,
    "axes.facecolor": "whitesmoke",
    "font.family": "serif",
    "figure.figsize":(20, 5)
    })

    lab_f = lambda e:labels.get(e, e)

    pos = np.array(range(len(df.index)))
    fig, ax = plot.subplots()
    fig.set_size_inches((12, 7))
    ax.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    ax.set_xticks(pos)
    ax.set_xticklabels(df.index)
    if len(keys) == 1:
        ax.bar(pos, df[keys[0]], width=width, zorder=4, label=lab_f(keys[0]), color=colors[keys[0]])
    else: 
        w = width/len(keys)
        for i, k in enumerate(keys):
            npos = pos + (len(keys)-1)*w*((i/(len(keys)-1))-0.5)
            ax.bar(npos, df[k], width=w, zorder=4, label=lab_f(k), color=colors[k])
    box = ax.get_position()
    q = math.ceil(len(keys)/3)
    if len(keys) < 4:
        hfactor = 0.9
        hanchor = 1.2
        ncol = 3
    else:
        hfactor = 0.8
        hanchor = 1.45
        ncol = 2
    ax.set_position([box.x0, box.y0, box.width, box.height*hfactor])
    ax.legend(loc='upper center', bbox_to_anchor=(0.5, hanchor),
          ncol=ncol)
    fig.tight_layout()
    return fig

def generate_graphs_csv(path, output):
    import pandas as pd
    df0 = pd.read_csv(path).set_index("id")
    generate_graphs(df0, output)

def generate_graphs_exp(path, outpath):
    import charts.extract_info as ei
    df0 = ei.exp_to_dataframe(path).set_index("id")
    df0.to_csv(outpath+"/data.csv")
    generate_graphs(df0, outpath)

def generate_graphs(df0, outpath):
    import charts.extract_info as ei

    df = df0[["jsurfer", "jsonski", "rsonpath"]].rename(dict(rsonpath="simdpath"), axis=1).drop("N1")

    df1 = df.filter(items=ei.jsonski_vs_rsonpath, axis=0)
    fig = plot_from_dataframe(df1)
    fig.savefig(outpath+"/simdpath_vs_jsonski.png", bbox_inches='tight')

    query_orig = list(map(lambda e:e[:-1], ei.query_rewritten))
    df2 = df.filter(items=query_orig, axis=0)
    df3 = df.filter(items=ei.query_rewritten, axis=0)[["simdpath", "jsurfer"]]
    df2[["rewritten_s", "rewritten_j"]] = df3.rename(lambda e:e[:-1])
    df2 = df2[["jsurfer", "rewritten_j", "jsonski", "simdpath", "rewritten_s"]]
    fig = plot_from_dataframe(df2)
    fig.savefig(outpath+"/query_rewritten.png", bbox_inches='tight')

    df4 = df.filter(items=ei.query_interest, axis=0)[["jsonski", "simdpath"]] 
    jsonski = df4[["jsonski"]]
    for i in ("Ts2", "Ts3"):
        jsonski = jsonski.drop(i)
    df4["jsonski"] = jsonski
    fig = plot_from_dataframe(df4)
    fig.savefig(outpath+"/query_interest.png", bbox_inches='tight')
