import { Sidebar, Menu, MenuItem } from 'react-pro-sidebar';
import { Link } from 'react-router-dom';

export default function SideBar() {
  return (
    <Sidebar style={{ backgroundColor: 'red' }}>
      <Menu>
        <MenuItem component={<Link to="/Dashboard" />}> DashBoard </MenuItem>
        <MenuItem> Council </MenuItem>

        <MenuItem> Curator </MenuItem>
        <MenuItem component={<Link to="/employee" />}> Employee </MenuItem>
      </Menu>
    </Sidebar>
  );
}
