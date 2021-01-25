pub(super) const BASE_STYLE: &str = "
#session_name {
    font-weight: bold;
}

#lap-times {
    font-weight: bold;
}

#throttle .filled {
    background-color: #00A000;
}

#brake .filled {
    background-color: #A00000;
}

#gear {
    font-size: 3em;
}
.pedal_input label {
    font-weight: bold;
}

.pedal_input block {
    border-style: none;
    border-top-left-radius: 5px;
    border-top-right-radius: 5px;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
}

.pedal_input trough {
    border-style: none;
    margin: 5px 0px;
}

infobar {
    border-style: none;
}

infobar.info {
    background-color: #3c6e3c;
}

infobar.question {
    background-color: #2a6e6e;
}

infobar.warning {
    background-color: #606026;
}

infobar.error {
    background-color: #734848;
}
";
