package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Collection;
import java.util.List;

public class Step2Choco {
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

    private static Collection<Integer> findCombinationComplete(Machine machine) {
        

        
    }
}
