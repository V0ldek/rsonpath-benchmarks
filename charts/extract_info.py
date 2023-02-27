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
    return bench.strip(), query.strip(), name.strip()

def process_exp_data(data):
    d2 = {}
    for e,v in data.items():
        d2[e] = h = {}
        for x in v:
            t = v[x]["throughput"]
            if not t:
                continue
            size = t.get("BytesDecimal", t.get("Bytes"))
            stdev = v[x]["estimates"]["median"][1]
            median = v[x]["estimates"]["median"][0]
            h[x] = size/median #(size/(median+stdev), size/median, size/(median-stdev))
    return d2


def exp_to_list(path:str):
    data = get_exp_data(path)
    processed = process_exp_data(data)
    L = []
    for e, v in processed.items():
        if e[0] != ".":
            continue
        t = format_bench(e)
        x, y, z = v["rsonpath"], v.get("jsonski"), v.get("jsurfer")
        qid = id_map[t[1]]
        query = id_queries[qid]     
        L.append((qid, t[1], t[0], query, x, y, z))

    L.sort(key=lambda e:e[:2])
    return L

header = ["id", "rsonpath_id", "dataset", "query", "rsonpath", "jsonski", "jsurfer"]

id_map = {
    "decl_name" : "A1",
    "included_from" : "A3",
    "nested_inner" : "A2",
    "BB1_products_category" : "B1",
    "BB1'_products_category" : "B1r",
    "BB2_products_video" : "B2",
    "BB2'_products_video" : "B2r",
    "BB3_products_video_only" : "B3",
    "BB3'_products_video_only" : "B3r",
    "scalability_affiliation0" : "S0",
    "scalability_affiliation1" : "S1",
    "author_affiliation" : "C1",
    "author_affiliation_descendant" : "C1r",
    "DOI" : "C0",
    "editor" : "C2",
    "editor_descendant" : "C2r",
    "orcid" : "C4",
    "orcid_descendant" : "C4r",
    "scalability_affiliation2" : "S2",
    "title" : "C3",
    "title_descendant" : "C3r",
    "scalability_affiliation4" : "S4",
    "GMD1_routes" : "G1",
    "GMD2_travel_modes" : "G2",
    "GMD2'_travel_modes" : "G2r",
    "NSPL1_meta_columns" : "N1",
    "NSPL2_data" : "N2",
    "added_counties_tags" : "O2",
    "added_countries_tags_descendant" : "O2r",
    "specific_ingredients" : "O3",
    "specific_ingredients_descendant" : "O3r",
    "vitamins_tags" : "O1",
    "vitamins_tags_descendant" : "O1r",
    "all_hashtags" : "Ts1",
    "hashtags_of_retweets" : "Ts2",
    "metadata_1" : "Ts3",
    "metadata_2" : "Ts4",
    "metadata_3" : "Ts5",
    "TT1_entities_urls" : "T1",
    "TT2_text" : "T2",
    "WM1_items_price" : "W1",
    "WM1'_items_price" : "W1r",
    "WM2_items_name" : "W2",
    "WM2'_items_name" : "W2r",
    "WP1_claims_p150" : "Wi",
    "WP1'_claims_p150" : "Wir"
}

id_queries = {
    "A1": "$..decl.name",
    "A3": "$..loc.includedFrom.file",
    "A2": "$..inner..inner..type.qualType",
    "B1": "$.products[*].categoryPath[*].id",
    "B1r": "$..categoryPath..id",
    "B2": "$.products[*].videoChapters[*].chapter",
    "B2r": "$..videoChapters..chapter",
    "B3": "$.products[*].videoChapters",
    "B3r": "$..videoChapters",
    "S0": "$..affiliation..name",
    "S1": "$..affiliation..name",
    "C1": "$.items[*].author[*].affiliation[*].name",
    "C1r": "$..author..affiliation..name",
    "C0": "$..DOI",
    "C2": "$.items[*].editor[*].affiliation[*].name",
    "C2r": "$..editor..affiliation..name",
    "C4": "$.items[*].author[*].ORCID",
    "C4r": "$..author..ORCID",
    "S2": "$..affiliation..name",
    "C3": "$.items[*].title",
    "C3r": "$..title",
    "S4": "$..affiliation..name",
    "G1": "$[*].routes[*].legs[*].steps[*].distance.text",
    "G2": "$[*].available_travel_modes",
    "G2r": "$..available_travel_modes",
    "N1": "$.meta.view.columns[*].name",
    "N2": "$.data[*][*][*]",
    "O2": "$.products[*].added_countries_tags",
    "O2r": "$..added_countries_tags",
    "O3": "$.products[*].specific_ingredients[*].ingredient",
    "O3r": "$..specific_ingredients..ingredient",
    "O1": "$.products[*].vitamins_tags",
    "O1r": "$..vitamins_tags",
    "Ts1": "$..hashtags..text",
    "Ts2": "$..retweeted_status..hashtags..text",
    "Ts3": "$.search_metadata.count",
    "Ts4": "$..search_metadata.count",
    "Ts5": "$..count",
    "T1": "$[*].entities.urls[*].url",
    "T2": "$[*].text",
    "W1": "$.items[*].bestMarketplacePrice.price",
    "W1r": "$..bestMarketplacePrice.price",
    "W2": "$.items[*].name",
    "W2r": "$..name",
    "Wi": "$[*].claims.P150[*].mainsnak.property",
    "Wir": "$..P150..mainsnak.property"
}

jsonski_vs_rsonpath = [
    "B1",
    "B2",
    "B3",
    "G1",
    "G2",
    "N1",
    "N2",
    "T1",
    "T2",
    "W1",
    "W2",
    "Wi"
]

query_rewritten = [
    "B1r",
    "B2r",
    "B3r",
    "G2r",
    "W1r",
    "W2r",
    "Wir"
]

query_interest = [
    "A1",
    "A2",
    "C1",
    "C1r",
    "C2",
    "C2r",
    "Ts3",
    "Ts4",
    "Ts5"
]

def get_table():
    import texttable
    T=texttable.Texttable(max_width=0)
    T.header(header)
    T.set_chars([' ', '|', '|', '-'])
    T.set_deco(texttable.Texttable.VLINES|texttable.Texttable.HEADER|texttable.Texttable.BORDER)
    return T


def print_table_csv(path:str):
    import csv, sys

    L = exp_to_list(path)
    writer = csv.writer(sys.stdout)
    writer.writerow(header)
    writer.writerows(L)

def table_markdown(path:str):
    L = exp_to_list(path)
    T = get_table()
    for e in L:
        T.add_row(e)
    return "\n".join(T.draw().split("\n")[0:-1])

def exp_to_dataframe(path:str):
    L = exp_to_list(path)
    import pandas
    DF = pandas.DataFrame(L, columns=header)
    return DF

