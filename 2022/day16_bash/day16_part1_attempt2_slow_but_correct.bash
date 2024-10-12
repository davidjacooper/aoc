#!/bin/bash

start_valve="AA"

declare -a non_zero_valves=()
declare -A flow_rates=()

declare -a all_valves=()
declare -A tunnels=()

# Read input file and populate initial data structures.
while IFS=' =;' read _ valve _ _ _ rate _ _ _ _ connecting_valves; do
    if [ $rate -gt 0 ]; then
        non_zero_valves+=($valve)
    fi
    
    all_valves+=($valve)
    flow_rates[$valve]=$rate
    tunnels[$valve]=${connecting_valves//,/} # Remove ',' characters
done

echo "---"

echo "non_zero_valves=${non_zero_valves[*]}"
echo -n "flow_rates="
for v in ${non_zero_valves[*]}; do
    echo -n "$v=${flow_rates[$v]}; "
done
echo

echo "all_valves=${non_zero_valves[*]}"
echo -n "tunnels="
for v in ${all_valves[*]}; do
    echo -n "$v=${tunnels[$v]}; "
done
echo

echo "---"
echo




function calc_distance() {
    # Performs a breadth-first-search to find the optimal tunnel distance between two valves.
    # (Generally we do this for valves having non-zero flow -- the ones we care about -- and these
    # are not necessarily directly connected to one another.
    
    local valve1=$1
    local valve2=$2
    
    local -i distance=0
    local -A visited=()
    local current_list=$valve1
    
    while true; do
        local -A next_set=()
        local -a next_list=()
        
        if [ -z "$current_list" ]; then
            echo "!!! Cannot find distance from $valve1 to $valve2"
            exit 1
        fi
        
        for current_valve in $current_list; do
            if [ $current_valve == $valve2 ]; then
                return $distance
            fi
            
            visited[$current_valve]=1
            
            for connecting_valve in ${tunnels[$current_valve]}; do
                if [ -z ${visited[$connecting_valve]} ] && [ -z ${next_set[$connecting_valve]} ]; then
                    next_set[$connecting_valve]=1
                    next_list+=($connecting_valve)
                fi
            done
            
        done
        current_list="${next_list[*]}"
        distance+=1
    done
}

declare -A distances=()
declare -A saved_results=()
declare -A saved_routes=()

declare -i max_pressure_released=0
declare optimal_route=


declare -i pressure_return
declare route_return

function try_route() {
    # Recursively find the optimal route, starting at time $2 from location $4, given a set of
    # unopened valves in $5.
    
    local indent=$1
    local -i time=$2
    local route_so_far=$3
    local location=$4
    local unvisited=$5
    
    echo "${indent}try_route(): time=$time, route_so_far=$route_so_far, location=$location, unvisited='$unvisited'" 1>&2
    
    local -i max_pressure_released=0
    local best_route
    
    for next_valve in $unvisited; do
        local distance_key="${location}-${next_valve}"
        local distance=${distances[$distance_key]}
        
        if [ -z "$distance" ]; then
            calc_distance $location $next_valve
            distance=$?
            
            distances[$distance_key]=$dist
            distances["${next_valve}-${location}"]=$dist # Distance for reverse route
        fi
        
        # The time after moving to the next valve.
        local next_time=$(( time + distance + 1 ))
    
        # If >= 30, there's no point considering it
        if [ $next_time -lt 30 ]; then
        
            # Calculate the total pressure released under this scenario.
            local next_unvisited="${unvisited/$next_valve/}"

            local result_key="${next_time}-${next_valve}-${next_unvisited// /,}"
            local pressure_released=${saved_results[$result_key]}
            local route=${saved_routes[$result_key]}

            if [ -z "$pressure_released" ]; then
                try_route "$indent  " $next_time "${route_so_far}-${next_valve}" $next_valve "${next_unvisited}"
                pressure_released=$pressure_return
                route=$route_return
                saved_results[$result_key]=$pressure_released
                saved_routes[$result_key]=$route
            fi
            
            if [ $max_pressure_released -lt $pressure_released ]; then
                max_pressure_released=$pressure_released
                best_route=$route
            fi
        fi
    done
    
    echo "$indent  max_pressure_released=$max_pressure_released"
    max_pressure_released+=$(( ( 30 - time  ) * ${flow_rates[$location]} ))
    echo "$indent  extra max_pressure_released=$max_pressure_released"
    pressure_return=$max_pressure_released
    route_return="$location-$best_route"
}

try_route "" 0 $start_valve $start_valve "${non_zero_valves[*]/$start_valve/}"
echo
echo $pressure_return
echo $route_return


# Took 62 minutes to compute for 'input'

# Answer: 1728
# Route: AA-OS-QJ-QE-OQ-GJ-DV-KU-
