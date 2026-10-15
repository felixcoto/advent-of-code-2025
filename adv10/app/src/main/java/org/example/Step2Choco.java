package org.example;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.IntStream;

import org.chocosolver.solver.Model;
import org.chocosolver.solver.Solution;
import org.chocosolver.solver.search.strategy.Search;
import org.chocosolver.solver.variables.IntVar;

public class Step2Choco {
    public static long resolve(Path path) throws IOException {
        return resolve(Files.readAllLines(path).stream().map(Machine::parseLine).toList());
    }
    record MachineWithIndex(int index,Machine machine){};

    public static long resolve(List<Machine> machines) {
        return IntStream.range(0, machines.size())
                .mapToObj(index->new MachineWithIndex(index, machines.get(index)))
                .parallel()
                .map(Step2Choco::findCombination)
                .mapToInt(i->i)
                .sum();
    }

    public static int findCombination(MachineWithIndex machineWithIndex){
        var nano1=System.nanoTime();
        var result=findCombination(machineWithIndex.machine);
        var nano2=System.nanoTime();
        var duration=Duration.ofNanos(nano2-nano1);
        System.out.println(String.format("Step 2 linea %d resultado %d tiempo %d milisegundos",machineWithIndex.index,result,duration.toMillis()));
        return result;
    }

    public static int findCombination(Machine machine) {
        return findCombinationComplete(machine).stream().mapToInt(val->val).sum();
    }

    private static Collection<Integer> findCombinationComplete(Machine machine) {
        // Matriz A rectangular (botones x jolts)
        // Dimensión: buttons x jolts
        int[][] A = new int[machine.buttons().size()][machine.joltRequeriments().size()];
        IntStream.range(0,machine.buttons().size())
                .forEach(buttonNum->
                    machine.buttons().get(buttonNum).stream().forEach(jolt->A[buttonNum][jolt]=1)
                );

        // Vector B (1x2)
        int[] B = machine.joltRequeriments().stream().mapToInt(i->i).toArray();

        int m = A.length;      
        int n = A[0].length;    

        Model model = new Model("Minimizar suma de incognitas");

        // 1. Incógnitas enteras positivas estrictas (x_i >= 1)
        IntVar[] X = new IntVar[m];
        for (int i = 0; i < m; i++) {
            var maxLimit=machine.buttons().get(i).stream()
                    .map(jolt->machine.joltRequeriments().get(jolt))
                    .mapToInt(jolt->jolt)
                    .min()
                    .getAsInt();
            X[i] = model.intVar("x_" + i, 0, maxLimit);
        }

        // 2. Restricciones matriciales: X * A = B (una por cada columna de A)
        for (int j = 0; j < n; j++) {
            IntVar[] vars = new IntVar[m];
            int[] coeffs = new int[m];

            for (int i = 0; i < m; i++) {
                vars[i] = X[i];
                coeffs[i] = A[i][j];
            }

            model.scalar(vars, coeffs, "=", B[j]).post();
        }

        // 3. Variable objetivo: totalSum = sum(x_i)
        IntVar totalSum = model.intVar("totalSum", machine.joltRequeriments().stream().mapToInt(i->i).max().getAsInt(), machine.joltRequeriments().stream().mapToInt(i->i).sum());
        model.sum(X, "=", totalSum).post();

        // 4. Configurar minimización
        model.setObjective(Model.MINIMIZE, totalSum);

        // 5. Resolver buscando la solución óptima
        // Configurar estrategia
        model.getSolver().setSearch(Search.minDomLBSearch(X));
        model.getSolver().limitTime("60s");
        // Resolver
        Solution bestSolution = model.getSolver().findOptimalSolution(totalSum, false);
        

        List<Integer> result=new ArrayList<>();
        for(var index=0;index<m;index++){
            result.add(bestSolution.getIntVal(X[index]));
        }
        return result;

        /*if (bestSolution != null) {
            System.out.println("Solución óptima encontrada:");
            for (int i = 0; i < m; i++) {
                System.out.println("x_" + i + " = " + bestSolution.getIntVal(X[i]));
            }
            System.out.println("Suma mínima total: " + bestSolution.getIntVal(totalSum));
        } else {
            System.out.println("No existe ninguna combinación de enteros positivos que cumpla el sistema.");
        }*/
    }

        
    
}
