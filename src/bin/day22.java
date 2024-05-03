// usage: java day22.java <file>
package bin;

import static java.lang.Long.parseLong;

import java.io.BufferedReader;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.ListIterator;
import java.util.Optional;
import java.util.regex.Pattern;
import java.util.stream.Stream;

public class day22 {
    private static void die(Exception e) {
        e.printStackTrace(System.err);
        System.exit(1);
    }

    private static void die(String message) {
        System.err.println(message);
        System.exit(1);
    }

    record Interval(long from, long to) {
        Optional<Interval> intersection(Interval other) {
            long from = Long.max(this.from, other.from);
            long to = Long.min(this.to, other.to);
            return (from > to) ? Optional.empty() : Optional.of(new Interval(from, to));
        }

        boolean contains(Interval other) {
            return other.from >= from && other.to <= to;
        }

        List<Interval> splitAround(Interval other) {
            if (this.equals(other)) {
                return List.of();
            }
            return this.intersection(other).map(common -> {
                List<Interval> splits = new ArrayList<>();
                if (common.from > this.from) {
                    splits.add(new Interval(this.from, common.from - 1));
                }
                if (common.to < this.to) {
                    splits.add(new Interval(common.to + 1, this.to));
                }
                return splits;
            }).orElseGet(() -> List.of());
        }
    }

    record Box(Interval x, Interval y, Interval z) {
        long volume() {
            return (x.to - x.from + 1) * (y.to - y.from + 1) * (z.to - z.from + 1);
        }

        boolean contains(Box other) {
            return x.contains(other.x) && y.contains(other.y) && z.contains(other.z);
        }

        Optional<Box> intersection(Box other) {
            Optional<Interval> x = this.x.intersection(other.x);
            Optional<Interval> y = this.y.intersection(other.y);
            Optional<Interval> z = this.z.intersection(other.z);
            return (x.isEmpty() || y.isEmpty() || z.isEmpty())
                    ? Optional.empty()
                    : Optional.of(new Box(x.get(), y.get(), z.get()));
        }

        boolean hasIntersection(Box other) {
            return intersection(other).isPresent();
        }

        List<Box> splitAround(Box other) {
            if (this.equals(other)) {
                return List.of();
            }
            return this.intersection(other).map(common -> {
                var splitsX = x.splitAround(common.x);
                var splitsY = y.splitAround(common.y);
                var splitsZ = z.splitAround(common.z);
                List<Box> splits = new ArrayList<>();
                for (var splitX : splitsX) {
                    splits.add(new Box(splitX, common.y, common.z));
                }
                for (var splitY : splitsY) {
                    splits.add(new Box(common.x, splitY, common.z));
                }
                for (var splitZ : splitsZ) {
                    splits.add(new Box(common.x, common.y, splitZ));
                }
                for (var splitX : splitsX) {
                    for (var splitY : splitsY) {
                        splits.add(new Box(splitX, splitY, common.z));
                    }
                }
                for (var splitX : splitsX) {
                    for (var splitZ : splitsZ) {
                        splits.add(new Box(splitX, common.y, splitZ));
                    }
                }
                for (var splitZ : splitsZ) {
                    for (var splitY : splitsY) {
                        splits.add(new Box(common.x, splitY, splitZ));
                    }
                }
                for (var splitX : splitsX) {
                    for (var splitY : splitsY) {
                        for (var splitZ : splitsZ) {
                            splits.add(new Box(splitX, splitY, splitZ));
                        }
                    }
                }
                return splits;
            }).orElseGet(() -> List.of());
        }
    }

    record LitBox(boolean on, Box box) {
    }

    static List<Box> process(List<LitBox> litBoxes) {
        List<Box> ons = new LinkedList<>();
        litBoxes.forEach(litBox -> {
            if (litBox.on) {
                var q = new ArrayDeque<Box>();
                q.add(litBox.box);
                while (!q.isEmpty()) {
                    Box box = q.poll();
                    ons.stream().filter(box::hasIntersection).findAny().ifPresentOrElse(on -> {
                        ons.remove(on);
                        q.addAll(on.splitAround(box));
                        q.addAll(box.splitAround(on));
                        q.add(box.intersection(on).get());
                    }, () -> ons.add(box));
                }
            } else {
                for (ListIterator<Box> it = ons.listIterator(); it.hasNext();) {
                    Box on = it.next();
                    if (on.hasIntersection(litBox.box)) {
                        it.remove();
                        on.splitAround(litBox.box).forEach(it::add);
                    }
                }
            }
        });
        return ons;
    }

    static Optional<Long> volume(List<Box> boxes) {
        return boxes.stream().map(Box::volume).reduce(Long::sum);
    }

    static long part1(List<LitBox> litBoxes) {
        Box outer = new Box(new Interval(-50, 50), new Interval(-50, 50), new Interval(-50, 50));
        List<Box> on = process(litBoxes.stream().filter(lb -> outer.contains(lb.box)).toList());
        return volume(on).get();
    }

    static long part2(List<LitBox> litBoxes) {
        List<Box> on = process(litBoxes);
        return volume(on).get();
    }

    static final Pattern LINE = Pattern
            .compile("^(on|off) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)");

    static Stream<LitBox> parse(Stream<String> lines) {
        return lines.map(line -> {
            var m = LINE.matcher(line);
            if (!m.matches()) {
                die("invalid input line: %s".formatted(line));
            }
            return new LitBox(
                    m.group(1).equals("on"),
                    new Box(new Interval(parseLong(m.group(2)), parseLong(m.group(3))),
                            new Interval(parseLong(m.group(4)), parseLong(m.group(5))),
                            new Interval(parseLong(m.group(6)), parseLong(m.group(7)))));
        });
    }

    public static void main(String[] args) {
        if (args.length != 1) {
            die("usage: java day22.java <file>");
        }
        try (InputStream input = Files.newInputStream(Path.of(args[0]))) {
            List<LitBox> litBoxes = parse(new BufferedReader(new InputStreamReader(input)).lines()).toList();
            System.out.println(part1(litBoxes));
            System.out.println(part2(litBoxes));
        } catch (Exception e) {
            die(e);
        }
    }
}
