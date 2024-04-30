import { Sidebar, Menu, MenuItem } from 'react-pro-sidebar';
import { Link } from 'react-router-dom';

export default function SideBar() {
  return (
    <Sidebar style={{ backgroundColor: 'red' }}>
      <Menu>
        <MenuItem component={<Link to="/Dashboard" />}> DashBoard </MenuItem>
        <MenuItem component={<Link to="/employee" />}> Employee </MenuItem>
        <MenuItem component={<Link to="/market" />}> Task Market </MenuItem>
        <MenuItem> Curator </MenuItem>
        <MenuItem component={<Link to="/council" />}> Council </MenuItem>
      </Menu>
    </Sidebar>
  );
}
