package org.example;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.BitSet;
import java.util.List;
import java.util.regex.Pattern;
import java.util.stream.IntStream;

public record Machine(BitSet objetive, List<List<Integer>> buttons, List<Integer>joltRequeriments) {
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

		return new Machine(objetive, buttons, joltRequirements);
	}
}
