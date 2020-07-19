def naive(n, arr)
  max = 0

  (0...n).to_a.permutation.each do |ord|
    score = 0

    ord.each.with_index(1) do |e, i|
      k, l, r = arr[e]
      if i <= k
        score += l
      else
        score += r
      end
    end

    if score > max
      max = score
    end
  end

  max
end

tests = [
  {
    n: 2,
    arr: [
      [1, 5 , 10],
      [2, 15,  5],
    ],
    expected: 25,
  },
  {
    n: 3,
    arr: [
      [2, 93, 78],
      [1, 71, 59],
      [3, 57, 96],
    ],
    expected: 221,
  },
]


tests.each do |t|
  actual = naive(t[:n], t[:arr])
  expected = t[:expected]
  if actual != expected
    puts "expected: #{expected}"
    puts "actual:   #{actual}"
  end
end

# random case
case_cnt = 100
n = 3
case_cnt.times do
  arr = n.times.map do
    k = rand(n) + 1
    l = rand(10)
    r = rand(10)
    [k, l, r]
  end
  expected = naive(n, arr)
  s = []
  s << "1"
  s << n
  arr.each do |e|
    s << e.join(' ')
  end
  File.write('in.txt', s.join("\n") + "\n")
  actual = `cargo run --bin e < in.txt`.chomp.to_i
  if expected != actual
    warn "expected: #{expected}"
    warn "actual  : #{actual}"
    exit 1
  end
end
