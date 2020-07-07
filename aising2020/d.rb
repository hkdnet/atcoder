n = gets.chomp.to_i
x = gets.chomp
xi = x.to_i(2)
xc = x.count('1')
xi1 = xi % (xc + 1)
xi2 = xi % (xc - 1)

def exec(n)
  c = 0
  tmp = n
  while tmp != 0
    c += 1 if tmp % 2 == 1
    tmp /= 2
  end

  n % c
end

memo = {}
memo[0] = 0
1.upto(n+1) do |n|
  tmp = n
  ans = 0
  while tmp != 0
    if memo[tmp]
      ans += memo[tmp]
      break
    else
      ans += 1
      tmp = exec(tmp)
    end
  end

  memo[n] = ans
end

solver = ->(i, del) do
  zero_p = x[i] == '0'
  num = zero_p ? xi1 + del : xi2 - del

  puts xi1
  puts del
  puts num

  if memo[num]
    memo[num]
  else
    if zero_p
      c = xc + 1
    else
      c = xc - 1
    end
    memo[num % c] + 1
  end
end

del = 2 ** n
n.times do |i|
  del /= 2
  puts solver.call(i, del)
end
