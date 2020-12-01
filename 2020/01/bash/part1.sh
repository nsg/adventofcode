#!/bin/bash

VALUES="$(cat - )"

for n1 in $VALUES; do
    for n2 in $VALUES; do
        if [[ $(( $n1 + $n2 )) == 2020 ]]; then
            echo $(( $n1 * $n2 ))
            break 2
        fi
    done
done
