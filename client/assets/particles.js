particlesJS("particles-js", {
    particles: {
      number: { value: 96, density: { enable: true, value_area: 1024 } },
      color: { value: "#c3c3c3" },
      shape: {
        type: "star",
        stroke: { width: 0, color: "#000000" },
        polygon: { nb_sides: 12 },
      },
      opacity: {
        value: 1,
        random: false,
        anim: { enable: false, speed: 1, opacity_min: 0.1, sync: false },
      },
      size: {
        value: 0.1,
        random: false,
        anim: { enable: false, speed: 40, size_min: 0.1, sync: false },
      },
      line_linked: {
        enable: true,
        distance: 208,
        color: "#c3c3c3",
        opacity: 1,
        width: 1,
      },
    },
    retina_detect: true,
  });
  