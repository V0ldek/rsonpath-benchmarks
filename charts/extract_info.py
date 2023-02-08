import os
import pathlib
import json
import numpy as np

rootpath = pathlib.Path(__file__).parent.parent


def collect_exps(path=None):
    path = pathlib.Path(rootpath, "target", "criterion") if path is None else path
    L = list(os.walk(path))
    L = list(filter(lambda e:"benchmark.json" in e[2] and "new" in e[0], L))
    exps = []
    for upath, _, docs in L:
        p = pathlib.Path(upath, "benchmark.json")
        with open(p) as f:
            d = json.load(f)
            exps.append(d)
        p = pathlib.Path(upath, "estimates.json")
        with open(p) as f:
            t = json.load(f)
            d["estimates"] = {
            "mean": [
                t["mean"]["point_estimate"],
                t["mean"]["standard_error"]
            ],
            "median": [
                t["median"]["point_estimate"],
                t["median"]["standard_error"]
            ]
            }
    return exps

def get_exp_data(path=None):
    exps = collect_exps(path=path)
    groups = {}
    for e in exps:
        fname = e["function_id"]
        if "_" in fname:
            for prog in ("rsonpath", "jsonski", "jsurfer"):
                if prog.lower() in fname:
                    fname = prog
        groups[e["group_id"]] = L = groups.get(e["group_id"], {})
        L[fname] = e
    return groups

def get_dataset(path=None):
    path = path if path else rootpath
    datapath = pathlib.Path(path, "data")
    it = os.walk(datapath)
    for directory,_,fs in it:
        for filename in fs:
            if filename.endswith(".json"):
                p = pathlib.Path(directory, filename)
                yield p

def get_query_names(path=None):
    d = get_exp_data(path=path)
    exps = list(sorted(d))
    exps_short = [f"{exps[i][0].upper()}{i}" for i in range(len(exps))]
    return exps_short, exps

def format_bench(name):
    a,b = name.split(".json_", maxsplit=1)
    bench = a.split("/")[-1]
    query = b
    return bench, query, name 

def process_exp_data(data):
    d2 = {}
    for e,v in data.items():
        d2[e] = h = {}
        for x in v:
            t = v[x]["throughput"]
            size = t.get("BytesDecimal", t.get("Bytes"))
            stdev = v[x]["estimates"]["median"][1]
            median = v[x]["estimates"]["median"][0]
            h[x] = size/median #(size/(median+stdev), size/median, size/(median-stdev))
    return d2


def get_table():
    import texttable
    T=texttable.Texttable(max_width=0)
    T.header(["dataset", "query", "rsonpath", "jsonski", "jsurfer", "rsonpath/jsonski", "rsonpath/jsurfer"])
    T.set_chars([' ', '|', '|', '-'])
    T.set_deco(texttable.Texttable.VLINES|texttable.Texttable.HEADER|texttable.Texttable.BORDER)
    return T

def print_table_markdown(path:str):
    data = get_exp_data(path)
    processed = process_exp_data(data)
    L = []
    for e, v in processed.items():
        t = format_bench(e)
        x, y, z = v["rsonpath"], v.get("jsonski"), v.get("jsurfer")
        r1 = x/y if y else 0
        r2 = x/z if z else 0
        L.append((t[0],t[1], x, y, z, r1, r2))

    L.sort(key=lambda e:e[:2])
    T = get_table()
    for e in L:
        T.add_row(e)
    return "\n".join(T.draw().split("\n")[0:-1])
