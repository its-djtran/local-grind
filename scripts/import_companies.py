"""Regenerates content/companies.json from liquidslr/leetcode-company-wise-problems.

usage: python3 scripts/import_companies.py <path-to-dataset-checkout>

Only slugs that appear in content/lists or content/problems are kept. For each (problem,
company) pair we record the all-time frequency and the most recent window the problem was
reported in (30d, 3m, 6m, 6m+, all).
"""
import csv
import json
import os
import re
import sys

WINDOWS = [
    ("1. Thirty Days.csv", "30d"),
    ("2. Three Months.csv", "3m"),
    ("3. Six Months.csv", "6m"),
    ("4. More Than Six Months.csv", "6m+"),
    ("5. All.csv", "all"),
]


def wanted_slugs(root):
    slugs = set(os.listdir(os.path.join(root, "content", "problems")))
    for name in os.listdir(os.path.join(root, "content", "lists")):
        if name.endswith(".toml"):
            text = open(os.path.join(root, "content", "lists", name)).read()
            slugs.update(re.findall(r'\[\s*"[^"]*",\s*"([^"]+)"\s*\]', text))
    return slugs


def slug_of(link):
    m = re.search(r"/problems/([^/?#]+)", link)
    return m.group(1).lower() if m else None


def main():
    dataset = sys.argv[1]
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    slugs = wanted_slugs(root)
    tags = {}
    for company in sorted(os.listdir(dataset)):
        folder = os.path.join(dataset, company)
        if not os.path.isdir(folder) or company.startswith("."):
            continue
        recency, frequency = {}, {}
        for filename, window in WINDOWS:
            path = os.path.join(folder, filename)
            if not os.path.isfile(path):
                continue
            with open(path, newline="", encoding="utf-8") as f:
                for row in csv.DictReader(f):
                    slug = slug_of(row.get("Link", ""))
                    if slug not in slugs:
                        continue
                    recency.setdefault(slug, window)
                    try:
                        freq = float(row.get("Frequency") or 0)
                    except ValueError:
                        freq = 0.0
                    if window == "all" or slug not in frequency:
                        frequency[slug] = freq
        for slug, window in recency.items():
            tags.setdefault(slug, []).append(
                {"company": company, "frequency": round(frequency.get(slug, 0.0), 1), "recency": window}
            )
    for slug in tags:
        rank = {w: i for i, (_, w) in enumerate(WINDOWS)}
        tags[slug].sort(key=lambda t: (rank[t["recency"]], -t["frequency"], t["company"]))
    out = os.path.join(root, "content", "companies.json")
    with open(out, "w") as f:
        json.dump(dict(sorted(tags.items())), f, indent=1, sort_keys=False)
        f.write("\n")
    print("wrote %d problems, %d tags -> %s" % (len(tags), sum(map(len, tags.values())), out))


if __name__ == "__main__":
    main()
