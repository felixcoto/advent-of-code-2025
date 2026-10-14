package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.NavigableMap;
import java.util.Set;
import java.util.stream.LongStream;

import org.jspecify.annotations.NonNull;

public class Step2 {

    public static long resolve(Path path) throws IOException {
        return resolve(Files.readAllLines(path).stream().map(Machine::parseLine).toList());
    }

    public static long resolve(List<Machine> machines) {
        var results=machines
                .stream()
                .mapToLong(Step2::findCombination)
                .boxed().toList();
       /*  if (results.size()!=170){
            throw new RuntimeException(String.format("Invalid number of results: %d",results.size()));
        }*/
        return results.stream().mapToLong(Long::valueOf).sum();
    }

    public static int findCombination(Machine machine) {
        return findCombinationComplete(machine).size();
    }

    public static List<List<Integer>> findCombinationComplete(Machine machine) {
        List<FindStateWithHeuristic> pendingStates = new ArrayList<>();
        Set<BitSet> viewedStates = new HashSet<>();
        for (var button : machine.buttons()) {
            var findState = new FindState(new BitSet(machine.objetive().length()), new ArrayList<>(), button);
            addStateToMap(machine, pendingStates, button, findState);
        }

        Collections.sort(pendingStates);
        while (true) {
            var findState = pendingStates.removeFirst().findState;
            var state = (BitSet) findState.state.clone();
            List<List<Integer>> buttonSequence=new ArrayList<>(findState.buttonSequence.size()+1);
            buttonSequence.addAll(findState.buttonSequence);
            buttonSequence.add(findState.button);
            findState.button.stream().forEach(pos -> state.flip(pos));
            if (machine.calculateDistance(state) == 0) {
                return buttonSequence;
            } else {
                if (viewedStates.add(state)) {
                    for (var button : machine.buttons()) {
                        var newFindState = new FindState(state, buttonSequence, button);
                        addStateToMap(machine, pendingStates, button, newFindState);
                    }
                    Collections.sort(pendingStates);
                }

            }
        }
    }

    public static List<FindStateWithHeuristic> expand(List<Integer> state, Machine machine, NavigableMap<Integer,List<FindStateWithHeuristic>> map){
        var joltState=machine.calculateJoltState(state);
        var currentDistance=machine.calculateDistanceJoltageFromButtonState(joltState);
        for (var index=0;index<machine.buttons().size();index++) {
            var button=machine.buttons().get(index);
            if (state.get(index)>0){
                var action=new Action(index, -1);
                var nextDistance=currentDistance-button.size();
                var findStateWithHeuristic=new FindStateWithHeuristic(nextDistance, new FindState(state, action));
                addStateToMap(findStateWithHeuristic,map);
            }

            
            var pushes=button.stream()
                    .mapToInt(pos->machine.joltRequeriments().get(pos)-joltState.get(pos))
                    .min();


            
        }
    }

    private static void addStateToMap(FindStateWithHeuristic findStateWithHeuristic,
            NavigableMap<Integer,List<FindStateWithHeuristic>> map) {
        map.computeIfAbsent(findStateWithHeuristic.heuristic, (_)-> new ArrayList<FindStateWithHeuristic>());
        map.get(findStateWithHeuristic.heuristic).add(findStateWithHeuristic);
    }

    private static void addStateToMap(Machine machine, List<FindStateWithHeuristic> pendingStates,
            List<Integer> button, @NonNull FindState findState) {
        var newCount = findState.buttonSequence.size() + (button.size() == machine.calculateDistance(findState.state) ? 1 : 2);
        pendingStates.add(new FindStateWithHeuristic(newCount, findState));
    }

    public static record FindState(List<Integer> state,  Action action) {

    }

    public static record FindStateWithHeuristic(int heuristic, FindState findState)
            implements Comparable<FindStateWithHeuristic> {

        @Override
        public int compareTo(FindStateWithHeuristic o) {
            return this.heuristic - o.heuristic;
        }
    }

    public static record Action(int buttonNumber, int pushCount){
        
        List<Integer> apply(List<Integer> state){
            var result=new ArrayList<>(state);
            var currentPushCount=state.get(buttonNumber);
            result.set(buttonNumber, currentPushCount+pushCount);
            return result;
        }
    }

}
