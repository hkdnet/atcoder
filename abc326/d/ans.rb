def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
R = gets.chomp.chars
C = gets.chomp.chars

def find
  arr = N.times.to_a

  pa = arr.permutation(N)
  pb = arr.permutation(N)
  pc = arr.permutation(N)

  pa.each do |a_pattern|
    pb.each do |b_pattern|
      pc.each do |c_pattern|
        f = true
        N.times.each do |i|
          if a_pattern[i] == b_pattern[i] ||
              b_pattern[i] == c_pattern[i] ||
              c_pattern[i] == a_pattern[i]
              f = false
            break
          end
        end
        next unless f

        r = N.times.map do |i|
          min = [
            a_pattern[i],
            b_pattern[i],
            c_pattern[i],
          ].min
          case min
          when a_pattern[i]
            ?A
          when b_pattern[i]
            ?B
          when c_pattern[i]
            ?C
          end
        end

        next if r != R

        tmp = N.times.flat_map {|i| [a_pattern[i], b_pattern[i], c_pattern[i]]}
        c = N.times.map do |i|
          idx = tmp.find_index i
          ["A", "B", "C"][idx % 3]
        end
        next if c != C

        return [a_pattern, b_pattern, c_pattern]
      end
    end
  end

  nil
end

ans = find

if ans
  b = N.times.map { N.times.map{ "." } }
  ["A", "B", "C"].each.with_index do |c, ci|
    ans[ci].each.with_index do |col, row|
      b[row][col] = c
    end
  end
  puts "Yes"
  puts(b.map {|cs|cs.join("")}.join("\n"))
else
  puts "No"
end
