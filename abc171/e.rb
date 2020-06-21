n = gets.chomp.to_i
aa = gets.chomp.split(' ').map(&:to_i)

sum = aa.reduce { |c, a| c ^ a }

puts aa.map { |a| sum ^ a }.join(' ')
