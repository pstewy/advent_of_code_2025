package main 

import "core:strconv"
import "core:strings"
import "core:fmt"
import "core:os"
import "core:sort"

main :: proc() {
    kitchen_db, err := read_file("input.txt") 
    if err != .None {
        fmt.println("failed reading file", err)
        panic("panic because I'm too lazy to convert the err to a string")
    }
    fmt.println(part_2(kitchen_db))
}

Range :: struct {
    start: int,
    end: int
}

KitchenDatabase :: struct {
    ranges: []Range,
    ingredients: []int
}

Error :: enum{
    None,
    Ingredient_Not_An_Int,
    Missing_New_Line,
    Range_Missing_Dash,
    Range_Val_Invalid,
}

part_1 :: proc(using db: KitchenDatabase) -> int {
    freshIDs: int
    for ingredient in ingredients {
        for range in ranges {
            if range.start <= ingredient && range.end >= ingredient {
                freshIDs += 1
                break
            }
        }
    } 
    return freshIDs
}

part_2 :: proc(using db: KitchenDatabase) -> int {
    consolidated_ranges: [dynamic]Range
    append(&consolidated_ranges, ranges[0])
    for i in 1..<len(ranges) {
        last := &consolidated_ranges[len(consolidated_ranges)-1]
        cur := ranges[i]
        if last.end >= cur.start {
            if cur.end >= last.end {
                fmt.println("consolidated", cur, "with", last)
                last.end = cur.end
            }
        } else {
            append(&consolidated_ranges, cur)
        }
    }
    
    totalIds: int
    for range in consolidated_ranges {
        fmt.println("range", range.start, range.end)
        res := (range.end - range.start) + 1 // add one for inclusivity
        if res < 0 {
            fmt.println("wtf", range)
        }
        totalIds += res
    }

    return totalIds 
}


read_file :: proc(filename: string) -> (db: KitchenDatabase, err: union { os.Error, Error }) {
    db = KitchenDatabase{};
    data := os.read_entire_file_from_filename_or_err(filename) or_return

    it := string(data)
    input := strings.split(it, "\n\n")
    if len(input) != 2 {
        return db, .Missing_New_Line
    }
    db.ranges= pull_accepted_values(input[0]) or_return
    db.ingredients = pull_ingredients(input[1]) or_return
    sort.quick_sort_proc(db.ranges, proc(a, b: Range) -> int {
        return a.start - b.start
    })
    return db, .None
}

pull_ingredients :: proc(raw_ingredients: string) -> (ingredients: []int, err: Error) {
    dyn: [dynamic]int
    for raw_ingredient in strings.split_lines(raw_ingredients) {
        ingredient, ok := strconv.parse_int(raw_ingredient)
        if !ok {
            return nil, .Ingredient_Not_An_Int
        }
        append(&dyn, ingredient)
    }
    return dyn[:], .None
}

pull_accepted_values :: proc(raw_ranges: string) -> (accepted_values: []Range, err: Error) {
    dyn: [dynamic]Range

    for range_vals in strings.split_lines(raw_ranges) {
        start, end := parse_range(range_vals) or_return
        append(&dyn, Range{start, end}) 
    }
    return dyn[:], .None
}

parse_range :: proc(range_vals: string) -> (start, end: int, err: Error) {
    parts := strings.split(range_vals, "-")
    if len(parts) != 2 {
        return 0, 0, .Range_Missing_Dash
    }
    raw_start, raw_end := parts[0], parts[1]
    ok: bool
    start, ok = strconv.parse_int(raw_start)
    if !ok {
        return 0, 0, .Range_Val_Invalid
    }
    end, ok = strconv.parse_int(raw_end)
    if !ok {
        return 0, 0, .Range_Val_Invalid
    }
    return start, end, .None
}