import csv

file_template = """layout: post.liquid
title: vinyl collection
description: A list of all the vinyl in my collection.
data:
    link_type: internal
    route: misc
    type: misc
---
I've been collecting vinyl since I got my <a href="http://www.project-audio.com/main.php?prod=debutcarbon">Pro-Ject Debut Carbon</a> in November 2016. My collection has grown steadily since then. Here are the albums I have, in lexicographic order by artist.

| **Artist** | **Album** |
| :-- | --- |"""

with open('vinyl.csv') as vinyl_csv:
    csv_reader = csv.DictReader(vinyl_csv, fieldnames=['artist', 'album'], delimiter='|')
    sorted_dict = list(sorted(csv_reader, key=lambda row:(row['artist'].lower(),row['album'].lower())))
    print(file_template)
    for row in sorted_dict:
        print('| ' + row['artist'] + ' | ' + row['album'] + ' |')
