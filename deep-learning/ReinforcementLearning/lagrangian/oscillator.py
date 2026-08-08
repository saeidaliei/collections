import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
import matplotlib.pyplot as plt


class PhysicsNN(nn.Module):
    def __init__(self):
        super(PhysicsNN, self).__init__()
        self.network = nn.Sequential(
            nn.Linear(1, 32),
            nn.Tanh(),
            nn.Linear(32, 32),
            nn.Tanh(),
            nn.Linear(32, 2)  # outputs position and velocity
        )

    def forward(self, t):
        return self.network(t)


def lagrangian(q, q_dot, k=1.0, m=1.0):
    """
    Lagrangian for simple harmonic oscillator
    L = T - V = (1/2)m(dx/dt)^2 - (1/2)kx^2
    """
    kinetic = 0.5 * m * q_dot**2
    potential = 0.5 * k * q**2
    return kinetic - potential


def compute_loss(model, t_batch, x0=1.0, v0=0.0):
    """
    Compute the physics-informed loss using the principle of least action
    and enforce initial conditions
    """
    t_batch.requires_grad_(True)

    # Get position and velocity predictions
    predictions = model(t_batch)
    q = predictions[:, 0]  # position
    q_dot = predictions[:, 1]  # velocity

    # Initial conditions loss
    ic_loss = (q[0] - x0)**2 + (q_dot[0] - v0)**2

    # Compute Lagrangian at each time point
    L = lagrangian(q, q_dot)

    # Compute action by integrating the Lagrangian
    # Using trapezoidal rule for numerical integration
    dt = t_batch[1] - t_batch[0]
    action = torch.trapz(L, t_batch.squeeze())

    # For least action principle, we want variations of the action to be zero
    # We can compute this by taking gradients with respect to the path
    action_gradient = torch.autograd.grad(action, model.parameters(), create_graph=True)
    action_loss = sum(torch.sum(grad**2) for grad in action_gradient)

    # Combine losses with weighting
    lambda_ic = 100.0  # Weight for initial conditions
    return action_loss + lambda_ic * ic_loss


# Training setup
model = PhysicsNN()
optimizer = optim.Adam(model.parameters(), lr=0.001)

# Generate time points for training
t = torch.linspace(0, 10, 100).reshape(-1, 1)

# Initial conditions
x0 = 1.0  # Initial position
v0 = 0.0  # Initial velocity

# Training loop
losses = []
n_epochs = 25000
for epoch in range(n_epochs):
    optimizer.zero_grad()
    loss = compute_loss(model, t, x0, v0)
    loss.backward()
    optimizer.step()
    losses.append(loss.item())

    if (epoch + 1) % 100 == 0:
        print(f'Epoch {epoch+1}, Loss: {loss.item():.6f}')

# Generate predictions
with torch.no_grad():
    predictions = model(t)
    positions = predictions[:, 0].numpy()
    velocities = predictions[:, 1].numpy()


# Plot results
# Calculate total energy at each point
def total_energy(q, q_dot, k=1.0, m=1.0):
    """
    Calculate total energy E = T + V
    where T = (1/2)mv² and V = (1/2)kx²
    """
    kinetic = 0.5 * m * q_dot**2
    potential = 0.5 * k * q**2
    return kinetic + potential


energies = total_energy(positions, velocities)

# Create subplot figure
plt.figure(figsize=(15, 5))

# Plot positions and velocities
plt.subplot(1, 3, 1)
plt.plot(t.detach().numpy(), positions, label='Position')
plt.plot(t.detach().numpy(), velocities, label='Velocity')
plt.xlabel('Time')
plt.ylabel('Value')
plt.title('Neural Network Predictions')
plt.legend()

# Plot phase space
plt.subplot(1, 3, 2)
plt.plot(positions, velocities)
plt.xlabel('Position')
plt.ylabel('Velocity')
plt.title('Phase Space Plot')

# Plot total energy
plt.subplot(1, 3, 3)
plt.plot(t.detach().numpy(), energies, 'r-', label='Total Energy')
plt.xlabel('Time')
plt.ylabel('Energy')
plt.title('Total Energy Over Time')
plt.legend()

print("Energy statistics:")
print(f"Mean energy: {np.mean(energies):.6f}")
print(f"Energy standard deviation: {np.std(energies):.6f}")
print(f"Relative energy fluctuation: {np.std(energies)/np.mean(energies)*100:.2f}%")

plt.tight_layout()
plt.show()
