require 'prime'

def factor_cnt(n)
  return 1 if n == 1
  arr = Prime.prime_division(n)
  arr.map(&:last).map { |e| e + 1 }.reduce(:*)
end

def naive(n)
  n * factor_cnt(n)
end

sum = 0
20.times do |n|
  n = n + 1
  a = naive(n+1)
  sum += a
  puts [n, a, sum].join(',')
end
puts sum
