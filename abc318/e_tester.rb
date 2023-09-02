def random_i
  [1, 2, 3, 4].sample
end
def solve
  `ruby e.rb < e_tmp.in`
end
def solve_naive
  `ruby e_naive.rb < e_tmp.in`
end
100.times do
  n = 10
  a = n.times.map { random_i }
  input = "#{n}\n"
  input += "#{a.join(" ")}\n"
  File.write("e_tmp.in", input)

  a1 = solve
  a2 = solve_naive
  puts "#{a1.chomp} vs #{a2.chomp}"
  if a1 != a2
    abort "wrong!!!"
  end
end

puts "ok"
