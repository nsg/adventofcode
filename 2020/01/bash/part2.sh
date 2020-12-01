#!/bin/bash

VALUES="$(cat - )"

for n1 in $VALUES; do
    for n2 in $VALUES; do
        for n3 in $VALUES; do
            if [[ $(( $n1 + $n2 + $n3 )) == 2020 ]]; then
                echo $(( $n1 * $n2 * $n3 ))
                break 3
            fi
        done
    done
done
