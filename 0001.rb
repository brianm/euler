puts (1..999).filter {|x| (x % 3 == 0) || (x % 5 == 0) }.inject(0){|a, x| a+x}
