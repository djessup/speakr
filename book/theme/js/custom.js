// Modern Theme JavaScript for mdBook
(function () {
  "use strict";

  // Initialize the theme
  function initTheme() {
    initCollapsibleSidebar();
    initThemeToggle();
    initSmoothScrolling();
    initAnimations();
  }

  // Collapsible Sidebar Functionality
  function initCollapsibleSidebar() {
    const sidebar = document.querySelector(".sidebar");
    if (!sidebar) return;

    // Find all list items that have sub-lists
    const chapterItems = sidebar.querySelectorAll(".chapter li");

    chapterItems.forEach((item) => {
      const link = item.querySelector("a");
      const subList = item.querySelector("ol");

      // Only process items that have sub-lists
      if (subList && link) {
        // Check if this item should be collapsible by default
        const shouldCollapse = loadExpandedState(getItemKey(link)) !== true;

        // Add collapse/expand classes and arrow
        item.classList.add(shouldCollapse ? "collapsed" : "expanded");

        // Add arrow indicator
        if (!link.querySelector(".collapse-arrow")) {
          const arrow = document.createElement("span");
          arrow.className = "collapse-arrow";
          arrow.textContent = shouldCollapse ? "▶" : "▼";
          link.appendChild(arrow);
        }

        // Set initial state
        if (shouldCollapse) {
          subList.style.maxHeight = "0px";
          subList.style.opacity = "0";
          subList.style.overflow = "hidden";
        }

        // Add click handler for toggle (only on arrow or if it's a non-navigable parent)
        const arrow = link.querySelector(".collapse-arrow");
        if (arrow) {
          arrow.addEventListener("click", function (e) {
            e.preventDefault();
            e.stopPropagation();
            toggleMenuItem(item, link, subList);
          });
        }

        // Also handle clicks on parent items that only serve as containers
        if (isParentOnly(link)) {
          link.addEventListener("click", function (e) {
            if (e.target === link || e.target === arrow) {
              e.preventDefault();
              e.stopPropagation();
              toggleMenuItem(item, link, subList);
            }
          });
        }
      }
    });
  }

  // Helper function to determine if this is a non-navigable parent item
  function isParentOnly(link) {
    // Check if the href points to an index or if it's mainly organizational
    const href = link.getAttribute("href");
    return href && (href.includes("index.html") || href.includes("#"));
  }

  // Generate a consistent key for storing state
  function getItemKey(link) {
    return link.textContent.trim().replace(/[\d\.]+\s*/, ""); // Remove numbers and dots
  }

  // Toggle menu item expanded/collapsed state
  function toggleMenuItem(item, link, subList) {
    const isExpanded = item.classList.contains("expanded");
    const arrow = link.querySelector(".collapse-arrow");

    if (isExpanded) {
      // Collapse
      item.classList.remove("expanded");
      item.classList.add("collapsed");

      // Animate collapse
      subList.style.maxHeight = "0px";
      subList.style.opacity = "0";

      if (arrow) arrow.textContent = "▶";

      saveExpandedState(getItemKey(link), false);
    } else {
      // Expand
      item.classList.remove("collapsed");
      item.classList.add("expanded");

      // Calculate height for smooth animation
      const scrollHeight = subList.scrollHeight;
      subList.style.maxHeight = scrollHeight + "px";
      subList.style.opacity = "1";

      if (arrow) arrow.textContent = "▼";

      saveExpandedState(getItemKey(link), true);
    }

    // Add animation effect
    subList.addEventListener("transitionend", function handler() {
      if (item.classList.contains("expanded")) {
        subList.style.maxHeight = "none";
      }
      subList.removeEventListener("transitionend", handler);
    });
  }

  // Save expanded state to localStorage
  function saveExpandedState(href, isExpanded) {
    try {
      const key = "mdbook-sidebar-expanded";
      const stored = JSON.parse(localStorage.getItem(key) || "{}");
      stored[href] = isExpanded;
      localStorage.setItem(key, JSON.stringify(stored));
    } catch (e) {
      console.warn("Could not save sidebar state:", e);
    }
  }

  // Load expanded state from localStorage
  function loadExpandedState(href) {
    try {
      const key = "mdbook-sidebar-expanded";
      const stored = JSON.parse(localStorage.getItem(key) || "{}");
      return stored[href];
    } catch (e) {
      console.warn("Could not load sidebar state:", e);
      return true; // Default to expanded
    }
  }

  // Enhanced theme toggle
  function initThemeToggle() {
    const themeToggle = document.querySelector("#theme-toggle");
    if (!themeToggle) return;

    themeToggle.addEventListener("click", function () {
      // Add animation class
      document.body.classList.add("theme-transitioning");

      setTimeout(() => {
        document.body.classList.remove("theme-transitioning");
      }, 300);
    });
  }

  // Smooth scrolling for anchor links
  function initSmoothScrolling() {
    document.querySelectorAll('a[href^="#"]').forEach((link) => {
      link.addEventListener("click", function (e) {
        const targetId = this.getAttribute("href").substring(1);
        const targetElement = document.getElementById(targetId);

        if (targetElement) {
          e.preventDefault();
          targetElement.scrollIntoView({
            behavior: "smooth",
            block: "start",
          });
        }
      });
    });
  }

  // Add intersection observer for animations
  function initAnimations() {
    if (!window.IntersectionObserver) return;

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add("animate-in");
          }
        });
      },
      {
        threshold: 0.1,
        rootMargin: "0px 0px -50px 0px",
      }
    );

    // Observe elements for animation
    document
      .querySelectorAll("h2, h3, p, blockquote, table, pre")
      .forEach((el) => {
        observer.observe(el);
      });
  }

  // Enhance search functionality
  function enhanceSearch() {
    const searchBar = document.querySelector("#searchbar");
    if (!searchBar) return;

    // Add search icon and clear button
    const searchContainer = searchBar.parentElement;
    searchContainer.style.position = "relative";

    // Add search enhancements here if needed
  }

  // Add keyboard shortcuts
  function initKeyboardShortcuts() {
    document.addEventListener("keydown", function (e) {
      // Escape key to close search
      if (e.key === "Escape") {
        const searchBar = document.querySelector("#searchbar");
        if (searchBar && document.activeElement === searchBar) {
          searchBar.blur();
        }
      }

      // Ctrl/Cmd + K to focus search
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        const searchBar = document.querySelector("#searchbar");
        if (searchBar) {
          searchBar.focus();
        }
      }
    });
  }

  // Add CSS animations
  function addAnimationStyles() {
    const style = document.createElement("style");
    style.textContent = `
            .theme-transitioning * {
                transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease !important;
            }

            .animate-in {
                opacity: 1;
                transform: translateY(0);
                transition: opacity 0.6s ease, transform 0.6s ease;
            }

            h2, h3, p, blockquote, table, pre {
                opacity: 0;
                transform: translateY(20px);
                transition: opacity 0.6s ease, transform 0.6s ease;
            }

            .animate-in {
                opacity: 1 !important;
                transform: translateY(0) !important;
            }
        `;
    document.head.appendChild(style);
  }

  // Initialize everything when DOM is ready
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", function () {
      // initTheme();
      enhanceSearch();
      // initKeyboardShortcuts();
      // addAnimationStyles();
    });
  } else {
    // initTheme();
    enhanceSearch();
    // initKeyboardShortcuts();
    // addAnimationStyles();
  }

  // Re-initialize on page navigation (for SPA-like behavior)
  window.addEventListener("popstate", function () {
    // setTimeout(initCollapsibleSidebar, 100);
  });
})();
