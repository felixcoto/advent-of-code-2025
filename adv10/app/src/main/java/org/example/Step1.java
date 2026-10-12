package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.LongStream;

import org.jspecify.annotations.NonNull;

public class Step1 {

    public static long resolve(Path path) throws IOException {
        return resolve(Files.readAllLines(path).stream().map(Machine::parseLine).toList());
    }

    public static long resolve(List<Machine> machines) {
        var results=machines
                .stream()
                .mapToLong(Step1::findCombination)
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

    private static void addStateToMap(Machine machine, List<FindStateWithHeuristic> pendingStates,
            List<Integer> button, @NonNull FindState findState) {
        var newCount = findState.buttonSequence.size() + (button.size() == machine.calculateDistance(findState.state) ? 1 : 2);
        pendingStates.add(new FindStateWithHeuristic(newCount, findState));
    }

    public static record FindState(BitSet state, List<List<Integer>> buttonSequence, List<Integer> button) {

    }

    public static record FindStateWithHeuristic(int heuristic, FindState findState)
            implements Comparable<FindStateWithHeuristic> {

        @Override
        public int compareTo(FindStateWithHeuristic o) {
            return this.heuristic - o.heuristic;
        }
    }
}
