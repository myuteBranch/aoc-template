gen-day day_num:
    cargo new day_{{day_num}}
    rm -rf day_{{day_num}}/src
    cp -r template/src day_{{day_num}}/src
    rm day_{{day_num}}/Cargo.toml
    cp template/Cargo.toml day_{{day_num}}/Cargo.toml
    sed -i 's/template/day_{{day_num}}/' day_{{day_num}}/Cargo.toml
    sed -i 's/template/day_{{day_num}}/' day_{{day_num}}/src/bin/part1.rs
    sed -i 's/template/day_{{day_num}}/' day_{{day_num}}/src/bin/part2.rs