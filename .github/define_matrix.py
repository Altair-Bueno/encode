import itertools
import tomllib
import json
from os import environ

def powerset(features):
    return itertools.chain.from_iterable(
        itertools.combinations(features, r)
        for r in range(len(features)+1)
    )

def flatten_dependencies(features, tree):
    res = []
    for feature in features:
        dependencies = tree.get(feature)
        if dependencies is None:
            continue
        elif dependencies != []:
            res += flatten_dependencies(dependencies, tree)
        res.append(feature)
    return res

def is_disjoint_combination(enables, tree):
    l = flatten_dependencies(enables, tree)
    return len(l) == len(set(l))

with open("Cargo.toml", "rb") as f:
    data = tomllib.load(f)

msrv = data['package']['rust-version']
del data["features"]["default"] # Default isn't really a feature, but rather a predefined set of features
features = list(data["features"].keys())

unique_disjoint_combinations = {}
for combination in powerset(features):
    flattened = flatten_dependencies(combination, data["features"])
    flattened.sort()
    if len(set(flattened)) == len(flattened):
        print(combination, flattened)
        unique_disjoint_combinations[tuple(flattened)] = combination

unique_disjoint_combinations = [
    ",".join(features)
    for features in unique_disjoint_combinations.values()
]

if "GITHUB_OUTPUT" in environ:
    with open(environ["GITHUB_OUTPUT"], "wt") as f:
        f.write(f"powerset={json.dumps(unique_disjoint_combinations)}\n")
        f.write(f"features={json.dumps(features)}\n")
        f.write(f"msrv={msrv}\n")
else:
    print(
        f"powerset={json.dumps(unique_disjoint_combinations)}",
        f"features={json.dumps(features)}",
        f"msrv={msrv}",
        sep="\n"
    )
