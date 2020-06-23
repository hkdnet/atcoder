def naive(n)
  min = 100
  # 100 a + 10b + c
  10.times do |a|
    10.times do |b|
      10.times do |c|
        m = 100*a + 10*b +c
        if m < n
          next
        end

        s = n - m
        tmp = a + b + c + s.to_s.split('').map(&:to_i).sum
        if tmp < min
          min = tmp
        end
      end
    end
  end

  min
end

400.times do |n|
  expected = naive(n)
  actual = `echo #{n} | cargo run --bin e`.to_i
  if expected != actual
    puts n
    puts "expected: #{expected}, actual: #{actual}"
    break
  end
end
