import os
import pathlib
import json
import sys
import matplotlib
import texttable
import matplotlib.pyplot as plot
import numpy as np
import math
import pandas as pd
from extract_info import *



if __name__ == "__main__":
    plot.rcParams.update({
        "font.size": 18,
        "axes.facecolor": "whitesmoke",
        "font.family": "serif"
    })
    path = None
    if len(sys.argv) > 1:
        path = pathlib.Path(sys.argv[1])
        if not path.is_dir():
            raise ValueError("Expect a path to a directory in input")
    d = get_exp_data(path=path)
    d2 = {}
    for e,v in d.items():
        if "rsonpath" not in v:
            continue
        d2[e] = h = {}
        for x in v:
            t = v[x]["throughput"]
            size = t.get("BytesDecimal", t.get("Bytes"))
            stdev = v[x]["estimates"]["median"][1]
            median = v[x]["estimates"]["median"][0]
            h[x] = size/median #(size/(median+stdev), size/median, size/(median-stdev))
    
    exps_short, exps = get_query_names(path=path) 
    jsurfer = np.array([d2[e].get("jsurfer", 0) for e in exps])
    rsonpath = np.array([d2[e].get("rsonpath", 0) for e in exps])
    jsonski = np.array([d2[e].get("jsonski", 0) for e in exps])
    width = 0.6
    ratio = 1.8

    pos = np.array(range(len(exps)))
    fig, (ax0, ax1) = plot.subplots(1, 2, gridspec_kw={'width_ratios':[1, ratio]})
    ax0.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    bar = ax0.bar(exps_short, jsurfer, width=width, label="jsurfer", color="tab:gray", zorder=3)
    ax0.legend()
    ax0.set_ylabel("GB/s")
    #ax0.bar_label(bar, [f"{e:0.2f}" for e in jsurfer])

    width = width/ratio

    bar = ax1.bar(pos+width/2+0.03, rsonpath, label="simdpath", width=width, color="tab:blue", zorder=3)
    ax1.set_xticks(pos)
    ax1.set_xticklabels(exps_short)
    ax1.bar_label(bar, [f"{e:0.0f}" for e in rsonpath/jsurfer])
    pos2, jsonski2 = zip(*filter(lambda e:e[1] > 0, zip(pos, jsonski)))
    jsonski2 = np.array(jsonski2)
    pos2 = np.array(pos2)

    bar = ax1.bar(pos2-width/2-0.03, jsonski2, label="jsonski", width=width, color="tab:red")
    ax1.bar_label(bar, [f"{e:0.0f}" for e in filter(bool, jsonski/jsurfer)], zorder=3)
    ax1.set_ylabel("GB/s")
    ax1.grid(color = 'white', linestyle = '-', linewidth = 3, zorder=1)
    ax1.legend()
    fig.tight_layout()
    fig.set_size_inches(20, 5)
    plot.subplots_adjust(wspace=0.2, left=0.06)
    plot.savefig("plot.png")
    sys.exit(0)
    queries = {}
    for e,v in d.items():
        if "rsonpath" not in v:
            continue
        queries[e] = h = {}
        for x in v:
            h[x] = v[x]["value_str"]
    T = texttable.Texttable()
    L = []
    L.append(("dataset", "problem", "engine", "queryname", "query"))
    dataset = None
    for i, exp in enumerate(exps):
        dataset2, problem = exp.split("_", maxsplit=1)
        if dataset != dataset2:
            dataset = dataset2
        d = queries[exp]
        for p, q in d.items():
            L.append((dataset, problem, p, exps_short[i], f"`{q}`"))
    

    cellsizes = [max(len(f[i])+4 if f[i] else 0 for f in L) for i in range(5)]
    n = sum(cellsizes)
    k = "".join("+"+"-"*n for n in cellsizes)+"+"
    print(k)
    for i, u in enumerate(L):
        u = list(v if v else '' for v in u)
        s = "|".join(f"{' '*((cellsizes[i] - len(v))//2)}{v}{' '*math.ceil((cellsizes[i]-len(v))/2)}" for i, v in enumerate(u))
        print(f"|{s}|")
        if i:
            print(k)
        else:
            print(k.replace('-', '='))
        

#    print("\n".join("\t".join(map(lambda e:e if e else "", e)) for e in L))

#    print("\n".join("\t".join(map(lambda e:e if e else "", e)) for e in L))
