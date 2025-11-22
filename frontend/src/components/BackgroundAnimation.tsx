import { motion, AnimatePresence } from "framer-motion";
import { useEffect, useState } from "react";

interface BackgroundAnimationProps {
  isActive: boolean;
}

const BackgroundAnimation = ({ isActive }: BackgroundAnimationProps) => {
  const [particles, setParticles] = useState<Array<{ id: number; x: number; y: number; size: number; duration: number; delay: number }>>([]);

  useEffect(() => {
    // Create particles only once on mount to avoid re-renders
    const newParticles = Array.from({ length: 30 }).map((_, i) => ({
      id: i,
      x: Math.random() * 100,
      y: Math.random() * 100 + 10, // Start slightly lower
      size: Math.random() * 3 + 1,
      duration: Math.random() * 3 + 2,
      delay: Math.random() * 2,
    }));
    setParticles(newParticles);
  }, []);

  return (
    <AnimatePresence>
      {isActive && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.8 }}
          className="absolute inset-0 z-0 overflow-hidden pointer-events-none"
        >
          {/* Pulsing Gradient Overlay */}
          <motion.div
            className="absolute inset-0 bg-gradient-to-br from-purple-900/30 via-blue-900/30 to-emerald-900/30"
            animate={{
              backgroundPosition: ["0% 0%", "100% 100%"],
              opacity: [0.3, 0.6, 0.3],
            }}
            transition={{
              duration: 8,
              repeat: Infinity,
              repeatType: "reverse",
              ease: "easeInOut",
            }}
            style={{ backgroundSize: "200% 200%" }}
          />

          {/* Floating Particles */}
          {particles.map((particle) => (
            <motion.div
              key={particle.id}
              className="absolute rounded-full bg-primary/40 blur-[1px]"
              style={{
                left: `${particle.x}%`,
                top: `${particle.y}%`,
                width: particle.size,
                height: particle.size,
              }}
              animate={{
                y: [0, -150], // Move up
                opacity: [0, 0.8, 0], // Fade in and out
                scale: [1, 1.5, 0.5],
              }}
              transition={{
                duration: particle.duration,
                repeat: Infinity,
                ease: "easeOut",
                delay: particle.delay,
              }}
            />
          ))}
          
          {/* Subtle Grid Effect (Optional, adds tech feel) - REMOVED */}

        </motion.div>
      )}
    </AnimatePresence>
  );
};

export default BackgroundAnimation;
