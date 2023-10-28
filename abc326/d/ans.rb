def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
R = gets.chomp.chars
C = gets.chomp.chars

$ans = nil

def solve(as, bs, cs, col, i, board)
  return board if i == N

  as.each.with_index do |a, ai|
    next unless a
    bs.each.with_index do |b, bi|
      next unless b
      cs.each.with_index do |c, ci|
        next unless c

        next if ai == bi || bi == ci || ci == ai

        next if col[ai] != nil && col[ai] != ?A
        next if col[bi] != nil && col[bi] != ?B
        next if col[ci] != nil && col[ci] != ?C

        min = [ai, bi, ci].min
        case R[i]
        when ?A
          next if min != ai
        when ?B
          next if min != bi
        when ?C
          next if min != ci
        end

        orig = board[i]
        board[i] = N.times.map do |i|
          case i
          when ai
            ?A
          when bi
            ?B
          when ci
            ?C
          else
            '.'
          end
        end
        orig_ca = col[ai]
        col[ai] = nil
        orig_cb = col[bi]
        col[bi] = nil
        orig_cc = col[ci]
        col[ci] = nil
        as[ai] = false
        bs[bi] = false
        cs[ci] = false

        ok = solve(
          as, bs, cs, col, i + 1, board
        )
        return ok if ok
        as[ai] = true
        bs[bi] = true
        cs[ci] = true
        col[ai] = orig_ca
        col[bi] = orig_cb
        col[ci] = orig_cc
        board[i] = orig
      end
    end
  end

  nil
end

b = solve(
  Array.new(N) { true },
  Array.new(N) { true },
  Array.new(N) { true },
  C.map {|e| e},
  0,
  N.times.map { N.times.map { '.' } }
)

if b
  puts "Yes"
  puts(b.map {|cs| cs.join("") }.join("\n"))
else
  puts "No"
end
