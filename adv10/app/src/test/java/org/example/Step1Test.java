package org.example;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Path;

import org.junit.jupiter.api.Test;

public class Step1Test {
    @Test 
    public void test_distance_2(){
        var machine=Machine.parseLine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        var pushCount=Step1.findCombination(machine);
        assertEquals(2, pushCount);
    }

    @Test 
    public void test_distance_1(){
        var machine=Machine.parseLine("[..#..#] (1,5) (0,2,3,4,5) (2,5) (2,3) {11,124,32,31,11,136}");
        var pushCount=Step1.findCombination(machine);
        assertEquals(1, pushCount);
    }

    @Test 
    public void test_distance_3(){
        var machine=Machine.parseLine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        var pushCount=Step1.findCombination(machine);
        assertEquals(3, pushCount);
    }


    @Test 
    public void test_distance_9(){
        var machine=Machine.parseLine("[...#.#..##] (1,3) (2,4,5,8,9) (2,6,8) (0,2,3,8) (0,3,5,6,9) (0,1,2,4,7,8,9) (0,4,8) (0,2,6,7,9) (0,1,2,3,4,6,8,9) (0,1,3,4,5,6) (0,4,5,7,8) (0,2,3,4,6,7,8,9) {101,50,87,58,84,41,77,47,96,76}");
        var pushCount=Step1.findCombinationComplete(machine);
        assertEquals(3, pushCount.size());
    }

    

    @Test 
    public void test_distance_test_file() throws IOException, URISyntaxException{
        var resource=this.getClass().getResource("/test.txt");
        
        var result=Step1.resolve(Path.of(resource.toURI()));
        assertEquals(7, result);
    }
}
