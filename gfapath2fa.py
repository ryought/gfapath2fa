#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import argparse
from Bio import SeqIO
from pathlib import Path
from collections import OrderedDict


def main():
    parser = argparse.ArgumentParser(description='')
    parser.add_argument('gfa', type=Path)
    args = parser.parse_args()
    parse_gfa(args.gfa)


def parse_w(s):
    """
    Parse W-line segment list.

    >>> parse_w('>hoge>fuga<hige')
    [('hoge', False), ('fuga', False), ('hige', True)]
    """
    return


def parse_p(s):
    """
    >>> parse_p('')
    """
    return


def parse_gfa(filename: str):
    segments = dict()
    paths = dict()
    with open(filename) as f:
        for line in f:
            tokens = line.split('\t')
            if tokens[0] == 'S':
                name = segments[1]
                sequence = segments[2]
                segments[name] = sequence
            elif segments[0] == 'P':
                name = segments[1]
                path = [(x[:-1], x[-1]) for x in segments[2].split(',')]
                paths[name] = path
            elif segments[0] == 'W':
                sample_id = segments[1]
                hap_id = segments[2]
                seq_id = segments[3]
                name = sample_id + '#' + hap_id + '#' + seq_id
                path = [(x[:-1], x[-1]) for x in segments[2].split(',')]
                paths[name] = path
    return segments, paths


if __name__ == '__main__':
    import doctest
    doctest.testmod()
    main()
