package org.example;


import java.io.IOException;
import java.lang.ref.Reference;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.NavigableMap;
import java.util.Set;
import java.util.TreeMap;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.Function;
import java.util.stream.Collectors;
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
        NavigableMap<Integer,List<StateWithHeuristic>> pendingStates = new TreeMap<>();
        Set<List<Integer>> viewedStates = new HashSet<>();
        
        {
            var initialState=new ArrayList<Integer>(machine.buttons().size());
            IntStream.range(0, machine.buttons().size()).forEach(_->initialState.add(0));
            var initialStateWithHeuristic=new StateWithHeuristic(estimate(initialState, machine), initialState);
            addStateToMap(initialStateWithHeuristic, pendingStates);
        }
       

        List<Integer> result=null;
        Integer resultPush=Integer.MAX_VALUE;
        while (true) {
            var key=pendingStates.firstKey();
            var buttonState=pendingStates.get(key).removeFirst().buttonState;
            if (pendingStates.get(key).isEmpty()){
                pendingStates.remove(key);
            }

            
            if (machine.calculateDistanceJoltageFromButtonState(buttonState)==0){
                /*var pushCount=newState.stream().mapToInt(val->val).sum();
                if (result==null || pushCount<resultPush){
                    result=newState;
                    resultPush=pushCount;
                }*/
               return buttonState;
            }else{
                expand(buttonState, machine, pendingStates, viewedStates);
                
            }
        }
        //return result;
    }

    public static void expand(List<Integer> state, Machine machine, NavigableMap<Integer,List<StateWithHeuristic>> map, Set<List<Integer>> viewedStates){
        var joltState=machine.calculateJoltState(state);
        var currentDistance=machine.calculateDistanceJoltageFromButtonState(state);
        for (var index=0;index<machine.buttons().size();index++) {
            var button=machine.buttons().get(index);
            if (state.get(index)>0){
                
                var action=new Action(index, -1);
                var newState=action.apply(state);
                if (viewedStates.add(newState)){
                    var StateWithHeuristic=new StateWithHeuristic(estimate(newState, machine), newState);
                    addStateToMap(StateWithHeuristic,map);
                }
            }

            
            var pushes=button.stream()
                    .mapToInt(pos->machine.joltRequeriments().get(pos)-joltState.get(pos))
                    .min()
                    .orElse(0);
            
            if (pushes>0){
                var action=new Action(index, pushes);
                var newState=action.apply(state);
                if (viewedStates.add(newState)){
                    var StateWithHeuristic=new StateWithHeuristic(estimate(newState, machine)+countPush(newState), newState);
                    addStateToMap(StateWithHeuristic,map);
                }
            }

            
        }
    }

    private static void addStateToMap(StateWithHeuristic StateWithHeuristic,
            NavigableMap<Integer,List<StateWithHeuristic>> map) {
        map.computeIfAbsent(StateWithHeuristic.heuristic, (_)-> new ArrayList<StateWithHeuristic>());
        map.get(StateWithHeuristic.heuristic).add(StateWithHeuristic);
    }
        
    public static record FindState(List<Integer> state) {

    }

    public static record StateWithHeuristic(int heuristic, List<Integer> buttonState)
            implements Comparable<StateWithHeuristic> {

        @Override
        public int compareTo(StateWithHeuristic o) {
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

    // public static int estimate(List<Integer> buttonState, Machine machine){
    //     var joltState=machine.calculateJoltState(buttonState);
    //     var pendingButtons=new ArrayList<>(machine.buttons());
    //     int estimation=0;
    //     record JoltStateAndPos(int pos,int joltState){};
    //     var pendingJoltRequirements=IntStream.range(0, joltState.size())
    //             .mapToObj(pos->new JoltStateAndPos(pos,machine.joltRequeriments().get(pos)-joltState.get(pos)))
    //             .collect(Collectors.toList());
    //     while (!pendingButtons.isEmpty() && !pendingJoltRequirements.isEmpty()){
    //         var joltStateOptional=pendingJoltRequirements.stream()
    //                 .max((j1,j2)->j1.joltState-j2.joltState);

    //         if (joltStateOptional.isEmpty()){
    //             break;
    //         }
    //         var joltStateAndPos=joltStateOptional.get();
    //         pendingJoltRequirements.removeIf(js->js.pos==joltStateAndPos.pos);
    //         estimation+=joltStateAndPos.joltState;
    //         pendingButtons.removeIf(button->button.contains(joltStateAndPos.pos));
            
    //     }
    //     return estimation;
    // }

    public static int estimate(List<Integer> buttonState, Machine machine){
        buttonState=new ArrayList<>(buttonState);
        var joltState=new AtomicReference<>(machine.calculateJoltState(buttonState));
        var pendingButtons=new ArrayList<>(machine.buttons());
        int estimation=0;

        record ButtonAndPushes(List<Integer> button, int pushes){};
        Set<Integer> affectedJolts=new HashSet<>();
        while (!pendingButtons.isEmpty()){
            joltState.set(machine.calculateJoltState(buttonState));
            var buttonAndPushes= pendingButtons.stream()
                .map(button->new ButtonAndPushes(button,button.stream()
                    .mapToInt(pos->machine.joltRequeriments().get(pos)-joltState.get().get(pos))
                    .min()
                    .orElse(0)))
                .max((bp1,bp2)->bp1.pushes-bp2.pushes)
                .get();


            if (affectedJolts.stream().noneMatch(jolt->buttonAndPushes.button.contains(jolt))){
                var action=new Action(machine.buttons().indexOf(buttonAndPushes.button),buttonAndPushes.pushes);
                buttonState=action.apply(buttonState);
                affectedJolts.addAll(buttonAndPushes.button);
                estimation+=action.pushCount;
            }
            
            pendingButtons.remove(buttonAndPushes.button);
            break;
        }
        return estimation;
    }
}
