package org.example;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.BitSet;
import java.util.List;
import java.util.Optional;
import java.util.regex.Pattern;
import java.util.stream.IntStream;
import java.util.stream.Stream;

import com.google.common.primitives.Ints;

public record Machine(int numberOfLights,BitSet objetive, List<List<Integer>> buttons, List<Integer>joltRequeriments) {
    public static Machine parseLine(String line){
		var objetiveFinalChar=line.indexOf("]");
		var objetiveChars=line.substring(1, objetiveFinalChar);
		BitSet objetive=new BitSet(objetiveChars.length());
		IntStream.range(0, objetiveChars.length())
		.forEach((pos)->{
			if (objetiveChars.charAt(pos)=='#'){
				objetive.set(pos);
			}
		});
		
		var buttonPattern=Pattern.compile("\\([0-9,]+\\)");
		var buttonMatcher=buttonPattern.matcher(line);
		List<List<Integer>> buttons = new ArrayList<List<Integer>>();
		while (buttonMatcher.find()){
			List<Integer> button=new ArrayList<>();
			var buttonSequence=buttonMatcher.group();
			buttonSequence=buttonSequence.substring(1,buttonSequence.length()-1);
			for (var lightButton:buttonSequence.split(",")){
				button.add(Integer.parseInt(lightButton));
			}
			buttons.add(button);
		}
		
		var joltPattern=Pattern.compile("(?<=\\{)[0-9,]+(?=\\})");
		var joltMatcher=joltPattern.matcher(line);
		joltMatcher.find();
		var joltStr=joltMatcher.group();
		var joltRequirements=Arrays.stream(joltStr.split(","))
				.map(Integer::parseInt)
				.toList();

		return new Machine(objetiveChars.length(), objetive, buttons, joltRequirements);
	}

	public int calculateDistance(BitSet otherState){
		return (int) IntStream.range(0, Math.max(objetive.length(), otherState.length()))
				.filter(pos->objetive.get(pos)!=otherState.get(pos))
				.count();
	}

	public int calculateDistanceJoltageFromButtonState(List<Integer>  buttonState){
		return calculateDistanceJoltageFromJoltState(calculateJoltState(buttonState));
		
	}

	public int calculateDistanceJoltageFromJoltState(List<Integer>  joltState){
		return IntStream.range(0, joltRequeriments.size())
			.map(pos->joltRequeriments.get(pos)-joltState.get(pos))
			.sum();
	}

	public List<Integer> calculateJoltState(List<Integer> buttonState) {
		var array=new Integer[joltRequeriments.size()];
		IntStream.range(0, array.length).forEach(pos->array[pos]=0);
		for (var buttonIndex=0;buttonIndex<buttonState.size();buttonIndex++){
			var button=buttons.get(buttonIndex);
			var buttonPushCount=buttonState.get(buttonIndex);
			for (var joltPos:button){
				array[joltPos]+=buttonPushCount;
			}
		}
		return Arrays.asList(array);
	}
}
