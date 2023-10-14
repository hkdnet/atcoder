def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

n, T = gets.chomp.split(" ")
N = n.to_i

possible = ->(s, t) {
  # p [s, t]
  next true if s == t

  if s.size == t.size
    # puts "replace"
    # replace
    f = false

    s.chars.zip(t.chars).each do |sc, tc|
      if sc != tc
        if f
          f = false
          break
        else
          f = true
        end
      end
    end

    f
  elsif s.size < t.size
    # puts "short"
    next false if s.size + 1 != t.size
    f = false
    0.upto(t.size) do |idx|
      s_idx = f ? idx - 1 : idx
      if s[s_idx] != t[idx]
        if f
          f = false
          break
        else
          f = true
        end
      end
    end

    f
  elsif s.size > t.size
    # puts "rev"
    possible.call(t, s)
  end
}

ans = []
N.times do |i|
  s = gets.chomp
  if possible[s, T]
    ans << (i + 1)
  end
end

puts ans.size
puts ans.join(" ")
