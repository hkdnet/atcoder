def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W = getis
m = H.times.map do
  gets.chomp.chars
end

start = nil
goal = nil
H.times do |h|
  W.times do |w|
    if m[h][w] == 'S'
      start = [h, w]
    elsif m[h][w] == 'G'
      goal = [h, w]
    end
  end
end

N = geti
dp = Hash.new do |h, k|
  h[k] = {}
end

N.times.map do

end
