#!/bin/bash

for i in {1..10}
do 
    cargo run --bin client -- --name "client-$i" &
    sleep 1
done

wait
