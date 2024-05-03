package bin;

import static java.lang.Integer.parseInt;

import java.io.BufferedReader;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.ListIterator;
import java.util.Optional;
import java.util.Queue;
import java.util.Set;
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

    record Interval(int from, int to) {
        @Override
        public String toString() {
            return "[%d, %d]".formatted(from, to);
        }

        Optional<Interval> intersection(Interval other) {
            int from = Integer.max(this.from, other.from);
            int to = Integer.min(this.to, other.to);
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
        @Override
        public String toString() {
            return "{x=%s, y=%s, z=%s}".formatted(x, y, z);
        }

        int volume() {
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

        List<Box> splitAround(Box other) {
            if (this.equals(other)) {
                return List.of(other);
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
            }).orElseGet(() -> List.of(this));
        }
    }

    record LitBox(boolean on, Box box) {
    }

    static final Pattern LINE = Pattern
            .compile("^(on|off) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)");

    static Stream<LitBox> parse(Stream<String> lines) {
        return lines.map(line -> {
            var m = LINE.matcher(line);
            if (!m.matches()) {
                die(new IllegalArgumentException("invalid input line: %s".formatted(line)));
            }
            return new LitBox(
                    m.group(1).equals("on"),
                    new Box(new Interval(parseInt(m.group(2)), parseInt(m.group(3))),
                            new Interval(parseInt(m.group(4)), parseInt(m.group(5))),
                            new Interval(parseInt(m.group(6)), parseInt(m.group(7)))));
        });
    }

    static Optional<Integer> volume(List<Box> boxes) {
        return boxes.stream().map(Box::volume).reduce(Integer::sum);
    }

    static List<Box> process(Stream<LitBox> litBoxes) {
        Box outer = new Box(new Interval(-50, 50), new Interval(-50, 50), new Interval(-50, 50));
        List<Box> ons = new LinkedList<>();
        litBoxes = litBoxes.filter(litBox -> outer.contains(litBox.box));
        litBoxes.forEach(litBox -> {
            // System.out.printf("processing: %s\n", litBox);
            if (litBox.on) {
                var q = new ArrayDeque<Box>();
                q.add(litBox.box);
                while (!q.isEmpty()) {
                    Box box = q.poll();
                    ons.removeIf(on -> box.contains(on));
                    if (ons.stream().anyMatch(on -> on.contains(box))) {
                        continue;
                    }
                    Optional<Box> intersects = ons.stream().filter(on -> on.intersection(box).isPresent()).findAny();
                    intersects.ifPresentOrElse(on -> {
                        ons.remove(on);
                        q.addAll(on.splitAround(box));
                        q.addAll(box.splitAround(on));
                        q.add(box.intersection(on).get());
                    }, () -> {
                        ons.add(box);
                    });
                }
            } else {
                ListIterator<Box> it = ons.listIterator();
                while (it.hasNext()) {
                    Box on = it.next();
                    if (on.intersection(litBox.box).isEmpty()) {
                        continue;
                    }
                    if (litBox.box.contains(on)) {
                        it.remove();
                        continue;
                    }
                    // System.out.printf("off intersection, removing %s\n", on);
                    it.remove();
                    for (Box split : on.splitAround(litBox.box)) {
                        // System.out.printf("adding split: %s\n", split);
                        it.add(split);
                    }
                }
            }
            // System.out.printf("new ons: %s, volume %s\n", ons, volume(ons));
            // System.out.println(volume(ons).get());
        });
        return ons;
    }

    public static void main(String[] args) {
        if (args.length != 1) {
            die("usage: java day22.java <file>");
        }
        try (InputStream input = Files.newInputStream(Path.of(args[0]))) {
            Stream<LitBox> litBoxes = parse(new BufferedReader(new InputStreamReader(input)).lines());
            List<Box> ons = process(litBoxes);
            Optional<Integer> sum = ons.stream().map(Box::volume).reduce(Integer::sum);
            System.out.println(sum);
        } catch (Exception e) {
            die(e);
        }
    }
}
