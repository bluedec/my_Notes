
const editWrapper = () => {
    let wrapper = document.getElementsByClassName("resolve");
    console.log(typeof(wrapper));
    wrapper.className += " test-class"
    console.log(wrapper);    
}

