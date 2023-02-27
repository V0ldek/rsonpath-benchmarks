import matplotlib.pyplot as plot
import numpy as np
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
    
def plot_from_dataframe(df, keys=None, width=0.8, colors=dict(simdpath="tab:blue", jsonski="tab:red", rewritten="tab:green")):
    keys = list(df) if not keys else keys
    plot.rcParams.update({
    "font.size": 28,
    "axes.facecolor": "whitesmoke",
    "font.family": "serif",
    "figure.figsize":(20, 5)
    })

    pos = np.array(range(len(df.index)))
    fig, ax = plot.subplots()
    fig.set_size_inches((12, 7))
    ax.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    ax.set_xticks(pos)
    ax.set_xticklabels(df.index)
    ax.set_ylabel("GB/s")
    if len(keys) == 1:
        ax.bar(pos, df[keys[0]], width=width, zorder=4, label=keys[0], color=colors[keys[0]])
    else: 
        w = width/len(keys)
        for i, k in enumerate(keys):
            npos = pos + (len(keys)-1)*w*((i/(len(keys)-1))-0.5)
            ax.bar(npos, df[k], width=w, zorder=4, label=k, color=colors[k])
    ax.legend()
    fig.tight_layout()
    return fig

def generate_graphs(path, outpath):
    import charts.extract_info as ei
    df0 = ei.exp_to_dataframe(path).set_index("id")
    df0.to_csv(outpath+"/data.csv")
    df = df0[["jsonski", "rsonpath"]].rename(dict(rsonpath="simdpath"), axis=1)

    df1 = df.filter(items=ei.jsonski_vs_rsonpath, axis=0).drop("N1")
    fig = plot_from_dataframe(df1)
    fig.savefig(outpath+"/simdpath_vs_jsonski.png")

    query_orig = list(map(lambda e:e[:-1], ei.query_rewritten))
    df2 = df.filter(items=query_orig, axis=0)
    df3 = df.filter(items=ei.query_rewritten, axis=0)[["simdpath"]]
    df2["rewritten"] = df3.rename(lambda e:e[:-1])
    fig = plot_from_dataframe(df2)
    fig.savefig(outpath+"/query_rewritten.png")

    df4 = df.filter(items=ei.query_interest, axis=0)[["simdpath"]] 
    fig = plot_from_dataframe(df4)
    fig.savefig(outpath+"/query_interest.png")
