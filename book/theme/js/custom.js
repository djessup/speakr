// Modern Theme JavaScript for mdBook
(function () {
  "use strict";

  // === Speakr custom light/dark toggle (replaces default theme menu) ===
  function initThemeToggle() {
    // Locate the existing mdBook theme button
    const themeBtn = document.getElementById("theme-toggle");
    if (!themeBtn) return; // already replaced

    // Hide the default theme list dropdown if present
    const themeList = document.getElementById("theme-list");
    if (themeList) {
      themeList.style.display = "none";
      themeList.remove();
    }

    // Remove default dropdown-related attributes
    themeBtn.removeAttribute("aria-haspopup");
    themeBtn.removeAttribute("aria-controls");
    themeBtn.removeAttribute("aria-expanded");
    themeBtn.title = "Toggle light / dark";
    themeBtn.setAttribute("aria-label", "Toggle light / dark");

    // Remove any existing click handlers by cloning
    const newBtn = themeBtn.cloneNode(true);
    themeBtn.parentNode.replaceChild(newBtn, themeBtn);

    // Ensure only one <i> inside the button that we will update
    function setIcon(isDark) {
      newBtn.innerHTML = `<i class="fa ${
        isDark ? "fa-moon-o" : "fa-sun-o"
      }"></i>`;
    }

    // Helper to clear mdBook theme classes
    function clearThemeClasses() {
      document.documentElement.classList.remove(
        "light",
        "dark",
        "ayu",
        "rust",
        "coal",
        "navy"
      );
    }

    // Initialise state – saved preference or system setting
    const storedPref = localStorage.getItem("mdbook-theme");
    let startingDark;
    if (storedPref === null) {
      // No saved preference – use the user’s OS setting
      startingDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    } else {
      startingDark = storedPref === "dark";
    }
    if (startingDark) {
      clearThemeClasses();
      document.documentElement.classList.add("dark", "ayu");
    } else {
      clearThemeClasses();
      document.documentElement.classList.add("light");
    }
    setIcon(startingDark);

    // Click handler
    newBtn.addEventListener("click", () => {
      const isCurrentlyDark = document.documentElement.classList.contains("dark");
      clearThemeClasses();
      if (isCurrentlyDark) {
        document.documentElement.classList.add("light");
        localStorage.setItem("mdbook-theme", "light");
        setIcon(false);
      } else {
        document.documentElement.classList.add("dark");
        localStorage.setItem("mdbook-theme", "dark");
        setIcon(true);
      }
      window.location.reload();
    });

  }
  // === End custom toggle ===

  // Initialize everything when DOM is ready
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", function () {
      initThemeToggle();
    });
  } else {
    initThemeToggle();
  }
})();
