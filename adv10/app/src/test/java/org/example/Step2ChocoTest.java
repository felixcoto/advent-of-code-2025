package org.example;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Path;

import org.junit.jupiter.api.Test;

public class Step2ChocoTest {
    @Test 
    public void test_distance_10(){
        var machine=Machine.parseLine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        var pushCount=Step2Choco.findCombination(machine);
        assertEquals(10, pushCount);
    }

    @Test 
    public void test_distance_12(){
        var machine=Machine.parseLine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        var pushCount=Step2Choco.findCombination(machine);
        assertEquals(12, pushCount);
    }

    @Test 
    public void test_distance_11(){
        var machine=Machine.parseLine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        var pushCount=Step2Choco.findCombination(machine);
        assertEquals(11, pushCount);
    }
    
    @Test 
    public void test_distance_test_file() throws IOException, URISyntaxException{
        var resource=this.getClass().getResource("/test.txt");
        
        var result=Step2Choco.resolve(Path.of(resource.toURI()));
        assertEquals(33, result);
    }
}
