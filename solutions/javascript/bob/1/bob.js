//
// This is only a SKELETON file for the 'Bob' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const hey = (message) => {
if (message.trim() === "") {
    return "Fine. Be that way!";

} else if (
    message.trim().endsWith("?") &&
    message === message.toUpperCase() &&
    /[A-Z]/.test(message)
) {
    return "Calm down, I know what I'm doing!";

} else if (message.trim().endsWith("?")) {
    return "Sure.";

} else if (
    message === message.toUpperCase() &&
    /[A-Z]/.test(message)
) {
    return "Whoa, chill out!";

} else {
    return "Whatever.";
}
};
