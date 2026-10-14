package org.example;


import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.NavigableMap;
import java.util.Set;
import java.util.TreeMap;
import java.util.stream.IntStream;

public class Step2 {

    public static long resolve(Path path) throws IOException {
        return resolve(Files.readAllLines(path).stream().map(Machine::parseLine).toList());
    }

    public static long resolve(List<Machine> machines) {
        var results=machines
                .stream()
                .mapToLong(Step2::findCombination)
                .peek(combination->System.out.println("Step2 result "+combination))
                .boxed().toList();
       /*  if (results.size()!=170){
            throw new RuntimeException(String.format("Invalid number of results: %d",results.size()));
        }*/
        return results.stream().mapToLong(Long::valueOf).sum();
    }

    public static int findCombination(Machine machine) {
        return findCombinationComplete(machine).stream().mapToInt(val->val).sum();
    }

    public static List<Integer> findCombinationComplete(Machine machine) {
        NavigableMap<Integer,List<FindStateWithHeuristic>> pendingStates = new TreeMap<>();
        Set<List<Integer>> viewedStates = new HashSet<>();
        
        {
            var initialState=new ArrayList<Integer>(machine.buttons().size());
            IntStream.range(0, machine.buttons().size()).forEach(_->initialState.add(0));
            var initialStateWithHeuristic=new FindStateWithHeuristic(machine.calculateDistanceJoltageFromButtonState(initialState), new FindState(initialState, new Action(0, 0)));
            addStateToMap(initialStateWithHeuristic, pendingStates);
        }
       

        List<Integer> result=null;
        Integer resultPush=Integer.MAX_VALUE;
        while (true) {
            var key=pendingStates.firstKey();
            var oldFindState=pendingStates.get(key).removeFirst().findState;
            if (pendingStates.get(key).isEmpty()){
                pendingStates.remove(key);
            }

            var newState=oldFindState.action.apply(oldFindState.state);
            if (machine.calculateDistanceJoltageFromButtonState(newState)==0){
                /*var pushCount=newState.stream().mapToInt(val->val).sum();
                if (result==null || pushCount<resultPush){
                    result=newState;
                    resultPush=pushCount;
                }*/
               return newState;
            }else{
                if (viewedStates.add(newState)){
                    expand(newState, machine, pendingStates);
                }
            }
        }
        //return result;
    }

    public static void expand(List<Integer> state, Machine machine, NavigableMap<Integer,List<FindStateWithHeuristic>> map){
        var joltState=machine.calculateJoltState(state);
        var currentDistance=machine.calculateDistanceJoltageFromButtonState(state);
        for (var index=0;index<machine.buttons().size();index++) {
            var button=machine.buttons().get(index);
            if (state.get(index)>0){
                var action=new Action(index, -1);
                var nextDistance=currentDistance+(button.size()*action.pushCount);
                var findStateWithHeuristic=new FindStateWithHeuristic(countPush(state)-1, new FindState(state, action));
                addStateToMap(findStateWithHeuristic,map);
            }

            
            var pushes=button.stream()
                    .mapToInt(pos->machine.joltRequeriments().get(pos)-joltState.get(pos))
                    .min()
                    .orElse(0);
            if (pushes>0){
                var action=new Action(index, pushes);
                var nextDistance=currentDistance+(button.size()*action.pushCount);
                var findStateWithHeuristic=new FindStateWithHeuristic(countPush(state)+pushes, new FindState(state, action));
                addStateToMap(findStateWithHeuristic,map);
            }

            
        }
    }

    private static void addStateToMap(FindStateWithHeuristic findStateWithHeuristic,
            NavigableMap<Integer,List<FindStateWithHeuristic>> map) {
        map.computeIfAbsent(findStateWithHeuristic.heuristic, (_)-> new ArrayList<FindStateWithHeuristic>());
        map.get(findStateWithHeuristic.heuristic).add(findStateWithHeuristic);
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

    public static int countPush(List<Integer> state){
       return  state.stream().mapToInt(val->val).sum();
    }
}
