(function() {var implementors = {};
implementors["libc"] = [];implementors["num"] = [];implementors["chrono"] = [];implementors["serde"] = [];implementors["task_hookrs"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()