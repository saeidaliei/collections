import torch
import torch.nn as nn
import torch.optim as optim
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


def compute_loss(model, t_batch):
    """
    Compute the physics-informed loss using the principle of least action
    S = ∫L dt should be stationary (δS = 0)
    """
    t_batch.requires_grad_(True)

    # Get position and velocity predictions
    predictions = model(t_batch)
    q = predictions[:, 0]  # position
    q_dot = predictions[:, 1]  # velocity

    # Compute Lagrangian at each time point
    L = lagrangian(q, q_dot)

    # Compute action by integrating the Lagrangian
    # Using trapezoidal rule for numerical integration
    dt = t_batch[1] - t_batch[0]
    action = torch.trapz(L, t_batch.squeeze())

    # For least action principle, we want variations of the action to be zero
    # We can compute this by taking gradients with respect to the path
    action_gradient = torch.autograd.grad(action, model.parameters(), create_graph=True)

    # The sum of squared gradients should be minimized
    return sum(torch.sum(grad**2) for grad in action_gradient)


# Training setup
model = PhysicsNN()
optimizer = optim.Adam(model.parameters(), lr=0.001)

# Generate time points for training
t = torch.linspace(0, 10, 100).reshape(-1, 1)

# Training loop
n_epochs = 10000
for epoch in range(n_epochs):
    optimizer.zero_grad()
    loss = compute_loss(model, t)
    loss.backward()
    optimizer.step()

    if (epoch + 1) % 100 == 0:
        print(f'Epoch {epoch+1}, Loss: {loss.item():.6f}')

# Generate predictions
with torch.no_grad():
    predictions = model(t)
    positions = predictions[:, 0].numpy()
    velocities = predictions[:, 1].numpy()

# Plot results
plt.figure(figsize=(12, 4))
plt.subplot(1, 2, 1)
plt.plot(t.detach().numpy(), positions, label='Position')
plt.plot(t.detach().numpy(), velocities, label='Velocity')
plt.xlabel('Time')
plt.ylabel('Value')
plt.title('Neural Network Predictions')
plt.legend()

# Plot phase space
plt.subplot(1, 2, 2)
plt.plot(positions, velocities)
plt.xlabel('Position')
plt.ylabel('Velocity')
plt.title('Phase Space Plot')

plt.tight_layout()
plt.show()
