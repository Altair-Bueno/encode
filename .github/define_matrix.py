from __future__ import annotations

import itertools
import tomllib
import json
import typing
import os
import logging
import sys


def powerset[T](features: typing.Sequence[T]) -> typing.Iterator[typing.Tuple[T, ...]]:
    """Return an iterator over the powerset of the given sequence.

    The powerset is the set of all possible subsets of the given sequence.

    Args:

        features: The sequence to generate the powerset for.

    Returns:
        An iterator over the powerset of the given sequence
    """
    return itertools.chain.from_iterable(
        itertools.combinations(features, r) for r in range(len(features) + 1)
    )


def resolve_dependencies[T: typing.Hashable](
    features: typing.Iterable[T], tree: typing.Mapping[T, typing.Sequence[T]]
) -> typing.Generator[T]:
    """Resolve the dependencies of the given features in the given tree.

    The tree is a mapping from features to their dependencies.

    This function is a generator that yields the features in the order they should be

    Args:
        features: The features to resolve the dependencies for.
        tree: The tree to resolve the dependencies in.

    Returns:
        A generator that yields the features in the order they should be."""
    for feature in features:
        dependencies = tree.get(feature)
        if dependencies is None:
            continue
        elif len(dependencies):
            yield from resolve_dependencies(dependencies, tree)
        yield feature


def all_unique[T: typing.Hashable](iterable: typing.Iterable[T]) -> bool:
    """Return True if all elements in the given iterable are unique, False otherwise.

    Args:
        iterable: The iterable to check for uniqueness.

    Returns:
        True if all elements in the given iterable are unique, False otherwise.
    """
    seen = set()
    for e in iterable:
        if e in seen:
            return False
        seen.add(e)
    return True


def main(manifest_path: str, output_file: typing.Optional[typing.IO] = None):
    """Generate a matrix definition for CI based on the given Cargo.toml file.

    The matrix definition in JSON is written to the given output file, or to stdout if no output file is given.

    Args:
        manifest_path: The path to the Cargo.toml file to generate the matrix for.
        output_file: The file to write the matrix definition to. If not given, stdout is used.
    """
    with open(manifest_path, "rb") as f:
        data = tomllib.load(f)

    msrv = data["package"].get("rust-version", "")
    logging.debug("Minimum supported Rust version: %s", msrv)

    # Default isn't really a feature, but rather a predefined set of features
    if "features" in data:
        logging.debug("Features found")
        try:
            del data["features"]["default"]
        except KeyError:
            logging.debug("No default feature set found")
        features = sorted(data["features"].keys())
        logging.debug("Features: %s", features)

        seen = set()
        unique_disjoint_combinations = []
        for combination in powerset(features):
            resolved = tuple(
                sorted(resolve_dependencies(combination, data["features"]))
            )
            if resolved not in seen and all_unique(resolved):
                seen.add(resolved)
                unique_disjoint_combinations.append(",".join(resolved))
        logging.debug("Unique disjoint combinations: %s", unique_disjoint_combinations)
    else:
        logging.debug("No features found")
        features = []
        unique_disjoint_combinations = []
    json.dump(
        {
            "powerset": unique_disjoint_combinations,
            "features": features,
            "msrv": msrv,
        },
        output_file or sys.stdout,
    )


if __name__ == "__main__":
    log_level = os.environ.get("LOG_LEVEL", "INFO").upper()
    logging.basicConfig(level=log_level)
    main(os.environ.get("CARGO_MANIFEST_PATH", "Cargo.toml"))
