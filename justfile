default:
    just --list

setup day:
    # prepare project for the day
    just scaffold {{day}}
    # download files for the day
    just download {{day}}
    # print the item
    just read {{day}}

scaffold day:
    cargo scaffold {{day}}

download day:
    cargo download {{day}}

debug day:
    cargo solve {{day}}

time day:
    cargo solve {{day}} --time --release

test day:
    cargo test --bin {{day}}

read day:
    cargo read {{day}}

submit day parts:
    cargo solve {{day}} --release --submit {{parts}}

