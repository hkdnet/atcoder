def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

M = geti
s1 = gets.chomp.split("").map(&:to_i)
s2 = gets.chomp.split("").map(&:to_i)
s3 = gets.chomp.split("").map(&:to_i)

def cnt(s, h)
  s.each.with_index do |i, index|
    h[i] ||= []
    h[i] << index
  end
  h
end

h1 = cnt(s1, {})
h2 = cnt(s2, {})
h3 = cnt(s3, {})

nums = h1.keys & h2.keys & h3.keys

hs = [h1, h2, h3]

solve = ->(num) {
  # puts "find #{num}"
  min = nil
  [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
  ].each do |arr|
    # puts "order"
    # p arr

    step = 0
    loop_cnt = 0
    arr.each do |index|
      # puts "step = #{step}, loop = #{loop_cnt}"
      a = hs[index][num]
      # p a
      next_step = a.find {|e| e >= step}
      if next_step
        step = next_step + 1
      else
        loop_cnt += 1
        step = 0
        next_step = a.first
        step = next_step + 1
      end
    end

    ans = M * loop_cnt + step - 1
    min ||= ans
    min = ans if min > ans
  end

  min
}

if nums.empty?
  puts "-1"
else
  min = nil

  # ans = solve[8]
  # puts "---"
  # puts ans
  # exit 0
  nums.each do |num|
    steps = solve[num]
    # p [      num, steps]
    min ||= steps
    min = steps if min > steps
  end

  puts min
end
