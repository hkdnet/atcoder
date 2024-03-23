def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp.chars.map { |c| c == '1' }
C = getis

def f(head)
  expected = head
  S.map do |c, index|
    ret = c != expected

    expected = !expected

    ret
  end
end

h = {}
[true, false].each do |head|
  flags = f(head)
  # acc[i]: i - 1 番目までその文字列にするときにかかるコスト
  acc = [0]
  C.each.with_index do |c, i|
    if flags[i]
      acc << acc.last + c
    else
      acc << acc.last
    end
  end
  h[head] = acc
end

# STDERR.puts h.inspect

ans = Float::INFINITY

(N-1).times do |k|
  # k 文字目と k+1文字目が同じとする
  [true, false].each do |head|
    # [0, k)
    left = h[head][k]
    if k % 2 == 0
      expected = head
    else
      expected = !head
    end
    # [k+2, N)
    right = h[!head][N] - h[!head][k+2]

    # STDERR.puts "k = #{k}, head = #{head ? 1 : 0} left = #{left} right = #{h[!expected][N]} - #{h[!expected][k+2]} => #{right}"

    tmp = left + right
    tmp += C[k]   if S[k] != expected
    tmp += C[k+1] if S[k+1] != expected

    ans = tmp if tmp < ans
  end
end

puts ans
