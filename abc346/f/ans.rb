def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp
T = gets.chomp
# N = 3, S=abc -> abcabcabc

def pos
  @pos ||= Hash.new do |h,k|
    h[k] = []
  end
end
S.chars.each.with_index do |c, i|
  pos[c] << i
end

def binary_search(l, r)
  loop do
    idx = (l + r)/2
    return idx if idx == l || idx == r
    f = yield idx
    if f
      l = idx
    else
      r = idx
    end
  end
end
def memo
  @memo ||= Hash.new do |h, k|
    h[k] = {}
  end
end
def mat
  @mat ||=
    begin
      h = Hash.new do |h,k|
        h[k] = Hash.new(S.size)
      end

      S.chars.each.with_index.reverse_each do |c, i|
        h[i+1].each do |cc, ii|
          h[i][cc] = ii
        end
        h[i][c] = i
      end

      h
    end
end
def next_char(c, idx)
  i = idx % S.size
  return nil if pos[c].empty?

  tmp = mat[i][c]
  if tmp == S.size
    tmp = mat[0][c] + S.size
  end
  d = tmp - i
  idx + d
end
LEN = S.size * N
max = (LEN / T.size) + 1
def ok?(k)
  # STDERR.puts "Testing #{k}"
  idx = 0
  T.chars.each do |c|
    k.times do
      nx = next_char(c, idx)
      return false if nx.nil?

      # STDERR.puts("next #{c} is #{nx}")
      if nx >= LEN
        return false
      end
      # p idx
      idx = nx + 1
    end
  end

  true
end
ans = binary_search(0, max) { |k| ok?(k) }
puts ans
