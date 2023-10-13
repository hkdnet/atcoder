def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
aa = getis
ordered = aa.map.with_index do |a, idx|
  [a, idx]
end.sort_by { |e| -e.first }
players = N.times.map do |i|
  id = i + 1
  cs = gets.chomp.split("")
  score = cs.zip(aa).map do |c, num|
    c == 'o' ? num : 0
  end.sum + id
  { id:, cs:, score: }
end

max_score = players.max_by { |e| e[:score] }[:score]

players.each do |player|
  rest = max_score - player[:score]
  cnt = 0
  idx = 0

  while rest > 0
    a, i = ordered[idx]
    if player[:cs][i] == 'x'
      rest -= a
      cnt += 1
    end

    idx += 1
  end

  puts cnt
end
