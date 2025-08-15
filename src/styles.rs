// Helper functions for conditional classes

pub fn radio_card_classes(is_selected: bool) -> Vec<&'static str> {
    let mut classes = vec![
        "rounded-2xl",
        "shadow-xl",
        "p-6",
        "flex",
        "flex-col",
        "items-center",
        "cursor-pointer",
        "transition-all",
        "border-2",
        "bg-white",
        "hover:shadow-2xl",
        "hover:scale-105",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-red-400",
    ];
    if is_selected {
        classes.extend([
            "border-red-600",
            "bg-red-50",
            "scale-105",
            "ring-2",
            "ring-red-200",
        ]);
    } else {
        classes.extend(["border-gray-200"]);
    }
    classes
}

pub fn radio_btn_classes(is_selected: bool, is_playing: bool) -> Vec<&'static str> {
    let mut classes = vec![
        "mt-2",
        "px-4",
        "py-2",
        "rounded-full",
        "font-bold",
        "text-white",
        "transition-colors",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-red-400",
        "shadow",
    ];
    if is_selected && is_playing {
        classes.push("bg-red-600");
    } else {
        classes.extend(["bg-red-400", "hover:bg-red-600"]);
    }
    classes
}

pub fn play_btn_classes(is_playing: bool) -> Vec<&'static str> {
    let mut classes = vec![
        "ml-4",
        "p-3",
        "rounded-full",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-red-400",
        "shadow",
    ];
    if is_playing {
        classes.push("bg-red-100");
    } else {
        classes.extend(["bg-gray-200", "hover:bg-red-100"]);
    }
    classes
}
